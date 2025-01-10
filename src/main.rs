use axum::extract::Path;
use axum::{
    self,
    extract::{Extension, MatchedPath, Query},
    http::Request,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::{get, post, put},
    Router,
};
use chrono::Utc;
use diesel::{prelude::*, r2d2::{self, ConnectionManager}, result::{DatabaseErrorKind, Error}};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{env, sync::Arc, time::Duration};
use thiserror::Error;
use tokio::net::TcpListener;
use tower_http::{classify::ServerErrorsFailureClass, cors::CorsLayer, trace::TraceLayer};
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod models;
pub mod schema;

use crate::models::{NewUser, UpdateUser, User};
use crate::schema::users;
use crate::schema::users::email;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let pool = Arc::new(establish_database_connection_pool());
    let app = Router::new()
        .route("/health", get(get_health))
        .route("/users/create", post(create_user))
        .route("/users", get(read_user))
        .route("/users/:id", put(update_user))
        .fallback(handler_404)
        .layer(Extension(pool.clone()))
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str)
                        .unwrap_or("unknown");
                    info_span!(
                        "Request received: ",
                        method = %request.method(),
                        matched_path = matched_path,
                        uri = %request.uri(),
                    )
                })
                .on_response(|response: &Response, latency: Duration, _span: &Span| {
                    tracing::info!("Response sent: status={:?}, latency={:?}", response.status(), latency);
                })
                .on_failure(
                    |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        tracing::error!("Request failed: {:?}", error);
                    },
                ),
        );

    let address = format!("{}:{}", env_host(), env_port());
    let listener = TcpListener::bind(&address).await.unwrap();
    tracing::info!("[Server]: listening on http://{}", address);
    axum::serve(listener, app).await.unwrap();
}

fn env_host() -> String {
    match env::var("HOST") {
        Ok(host) => host.to_string(),
        _ => "[::]".to_string(),
    }
}

fn env_port() -> String {
    match env::var("PORT") {
        Ok(port) => port.to_string(),
        _ => "8080".to_string(),
    }
}

pub fn establish_database_connection_pool() -> DbPool {
    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        _ => {
            panic!("DATABASE_URL must be set");
        }
    };
    let manager = ConnectionManager::<PgConnection>::new(database_url.clone());
    match r2d2::Pool::builder().build(manager) {
        Ok(connection) => {
            tracing::info!("[Database]: connected");
            connection
        }
        Err(err) => {
            tracing::error!("Could not connect to database, reason {}", err);
            panic!("Error connecting to {}", database_url)
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub data: Option<Value>,
    pub error: Option<Value>,
    pub success: bool,
}

pub fn build_response(status: StatusCode, data: Option<Value>, error: Option<Value>) -> impl IntoResponse {
    let body = ApiResponse { data, error: error.clone(), success: error.is_none() };
    (status, Json(body))
}

async fn get_health() -> impl IntoResponse {
    let body = json!({ "message": "OK" });
    build_response(StatusCode::OK, Some(body), None)
}

async fn handler_404() -> impl IntoResponse {
    let error_message = json!({ "message": "Nothing to see here" });
    build_response(StatusCode::NOT_FOUND, None, Some(error_message))
}

#[derive(Serialize, Debug, Error)]
pub enum ApiErrors {
    #[error("Validation error")]
    ValidationError,

    #[error("Unique constraint error")]
    UniqueConstraintViolationError,

    #[error("Database error")]
    DatabaseError,

    #[error("Internal server error")]
    InternalServerError,
}

#[derive(Serialize, Debug, Error)]
#[serde(rename_all = "PascalCase")]
pub enum UserErrors {
    #[error("Username already taken")]
    UsernameAlreadyTaken,

    #[error("Email already in use")]
    EmailAlreadyInUse,

    #[error("User not found")]
    UserNotFound,
}

#[derive(Serialize, Debug, Error)]
#[serde(untagged)]
pub enum AppErrors {
    #[error(transparent)]
    UserError(#[from] UserErrors),

    #[error(transparent)]
    ApiError(#[from] ApiErrors),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreatePayload {
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct UserCreatedResponse {
    id: i32,
}

async fn create_user(Extension(pool): Extension<Arc<DbPool>>, Json(payload): Json<Value>) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(error) => {
            tracing::error!("{}: Failed to get DB connection {}", ApiErrors::DatabaseError.to_string(), error);
            let error_message = json!({ "code": ApiErrors::InternalServerError.to_string() });
            return build_response(StatusCode::INTERNAL_SERVER_ERROR, None, Some(error_message));
        }
    };

    let new_user: NewUser = match serde_json::from_value::<UserCreatePayload>(payload) {
        Ok(data) => {
            NewUser {
                email: data.email.to_lowercase(),
                username: data.username.to_lowercase(),
                first_name: data.first_name.filter(|f| !f.trim().is_empty()),
                last_name: data.last_name.filter(|l| !l.trim().is_empty()),
                password: "SuperSecurePassword".to_string(),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            }
        }
        Err(error) => {
            let error_message = json!({ "message": error.to_string(), "code": ApiErrors::ValidationError.to_string() });
            return build_response(StatusCode::BAD_REQUEST, None, Some(error_message));
        }
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result::<User>(&mut *conn) {
        Ok(user) => {
            let body = json!(UserCreatedResponse { id: user.id });
            build_response(StatusCode::CREATED, Some(body), None)
        }
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info)) => {
            let error_code = match info.constraint_name() {
                Some("users_username_key") => "UsernameAlreadyTaken".to_string(),
                Some("users_email_key") => "EmailAlreadyInUse".to_string(),
                _ => "UniqueConstraintViolation.".to_string(),
            };
            build_response(StatusCode::CONFLICT, None, Some(json!({ "code": error_code })))
        }
        Err(error) => {
            tracing::error!("{}: Unexpected database error {}", ApiErrors::DatabaseError.to_string(), error);
            let error_message = json!({ "code": ApiErrors::InternalServerError.to_string() });
            build_response(StatusCode::INTERNAL_SERVER_ERROR, None, Some(error_message))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdatePayload {
    pub email: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct UserUpdatedResponse {
    id: i32,
}

async fn update_user(Extension(pool): Extension<Arc<DbPool>>, Path(user_id): Path<i32>, Json(payload): Json<Value>) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(error) => {
            tracing::error!("{}: Failed to get DB connection {}", ApiErrors::DatabaseError.to_string(), error);
            let error_message = json!({ "code": ApiErrors::InternalServerError.to_string() });
            return build_response(StatusCode::INTERNAL_SERVER_ERROR, None, Some(error_message));
        }
    };

    let update_user: UpdateUser = match serde_json::from_value::<UserUpdatePayload>(payload) {
        Ok(data) => {
            UpdateUser {
                email: data.email.map(|e| e.to_lowercase()),
                username: data.username.map(|u| u.to_lowercase()),
                first_name: data.first_name.filter(|f| !f.trim().is_empty()),
                last_name: data.last_name.filter(|l| !l.trim().is_empty()),
                updated_at: Utc::now().naive_utc(),
            }
        }
        Err(error) => {
            let error_message = json!({ "message": error.to_string(), "code": ApiErrors::ValidationError.to_string() });
            return build_response(StatusCode::BAD_REQUEST, None, Some(error_message));
        }
    };

    match diesel::update(users::table.find(user_id))
        .set(&update_user)
        .execute(&mut *conn) {
        Ok(rows_affected) if rows_affected == 0 => {
            let error_message = json!({ "code": UserErrors::UserNotFound, "message": UserErrors::UserNotFound.to_string() });
            build_response(StatusCode::NOT_FOUND, None, Some(error_message))
        }
        Ok(..) => {
            let body = json!(UserUpdatedResponse { id: user_id });
            build_response(StatusCode::OK, Some(body), None)
        }
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info)) => {
            let error_code = match info.constraint_name() {
                Some("users_username_key") => AppErrors::UserError(UserErrors::UsernameAlreadyTaken),
                Some("users_email_key") => AppErrors::UserError(UserErrors::EmailAlreadyInUse),
                _ => AppErrors::ApiError(ApiErrors::UniqueConstraintViolationError),
            };
            build_response(StatusCode::CONFLICT, None, Some(json!({ "code": error_code, "message": error_code.to_string() })))
        }
        Err(error) => {
            tracing::error!("{}: Unexpected database error {}", ApiErrors::DatabaseError.to_string(), error);
            let error_message = json!({ "code": ApiErrors::InternalServerError.to_string() });
            build_response(StatusCode::INTERNAL_SERVER_ERROR, None, Some(error_message))
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
struct ReadUserQueryParams {
    email: String,
}

async fn read_user(Extension(pool): Extension<Arc<DbPool>>, Query(params): Query<ReadUserQueryParams>) -> impl IntoResponse {
    if params.email.trim().is_empty() {
        let error_message = json!({
            "code": ApiErrors::ValidationError,
            "message": "Email is required and cannot be empty."
        });
        return build_response(StatusCode::CONFLICT, None, Some(error_message));
    }

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(error) => {
            tracing::error!("{}: Failed to get DB connection {}", ApiErrors::DatabaseError.to_string(), error);
            let error_message = json!({ "code": ApiErrors::InternalServerError.to_string() });
            return build_response(StatusCode::INTERNAL_SERVER_ERROR, None, Some(error_message));
        }
    };

    match users::table
        .filter(email.eq(params.email))
        .first::<User>(&mut conn) {
        Ok(user) => {
            let body = json!(user);
            build_response(StatusCode::OK, Some(body), None)
        }
        Err(Error::NotFound) => {
            let error_message = json!({ "code": UserErrors::UserNotFound, "message": UserErrors::UserNotFound.to_string() });
            build_response(StatusCode::NOT_FOUND, None, Some(error_message))
        }
        Err(error) => {
            tracing::error!("{}: Unexpected database error {}", ApiErrors::DatabaseError.to_string(), error);
            let error_message = json!({ "code": ApiErrors::InternalServerError.to_string() });
            build_response(StatusCode::INTERNAL_SERVER_ERROR, None, Some(error_message))
        }
    }
}
