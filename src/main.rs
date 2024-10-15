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

use crate::models::{NewUser, User};
use crate::schema::users;

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
        .route("/users/:id", get(read_user))
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

    #[error("Internal server error")]
    InternalServerError,
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
    let mut conn = pool.get().expect("Failed to get DB connection");

    let new_user: NewUser = match serde_json::from_value::<UserCreatePayload>(payload) {
        Ok(data) => {
            let first_name = Some(data.first_name.as_ref().map_or_else(|| "".to_string(), |s| s.to_string()));
            let last_name = Some(data.last_name.as_ref().map_or_else(|| "".to_string(), |s| s.to_string()));
            let password = "SuperSecurePassword".to_string();

            NewUser {
                email: data.email.to_lowercase(),
                username: data.username.to_string(),
                first_name,
                last_name,
                password,
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
            let error_message = json!({ "code": ApiErrors::InternalServerError.to_string() });
            tracing::error!("Internal server error {}", error);
            build_response(StatusCode::INTERNAL_SERVER_ERROR, None, Some(error_message))
        }
    }
}

async fn update_user(Json(payload): Json<Value>) -> impl IntoResponse {
    println!("{:?}", payload);
    let ok = json!({ "message": "OK" });
    build_response(StatusCode::OK, Some(ok), None)
}

#[derive(Deserialize, Clone, Debug)]
struct ReadUserQueryParams {
    email: String,
}

async fn read_user(Query(params): Query<ReadUserQueryParams>) -> impl IntoResponse {
    println!("{:?}", params);
    let body = json!({ "email": params.email });
    build_response(StatusCode::OK, Some(body), None)
}
