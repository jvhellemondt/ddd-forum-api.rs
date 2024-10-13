use std::{env, time::Duration};
use dotenv::dotenv;
use axum::{
    self,
    extract::MatchedPath,
    http::StatusCode,
    http::Request,
    response::{Response,IntoResponse, Json},
    routing::{get, post, put},
    Router,
};
use axum::extract::Query;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tower_http::{classify::ServerErrorsFailureClass, cors::CorsLayer, trace::TraceLayer};
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route("/health", get(get_health))
        .route("/users/create", post(create_user))
        .route("/users/:id", get(read_user))
        .route("/users/:id", put(update_user))
        .fallback(handler_404)
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
    tracing::info!("Server: Listening on http://{}", address);
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

#[derive(Debug, Serialize)]
pub struct ApiResponse<T, E> {
    pub data: Option<T>,
    pub error: Option<E>,
    pub success: bool,
}

pub fn build_response<T, E>(status: StatusCode, data: Option<T>, error: Option<E>) -> impl IntoResponse
where
    T: Serialize + Clone,
    E: Serialize + Clone,
{
    let body = ApiResponse { data, error: error.clone(), success: error.is_none() };
    (status, Json(body))
}

async fn get_health() -> impl IntoResponse {
    let body = json!({ "message": "OK" });
    build_response(StatusCode::OK, Some(body), None::<String>)
}

async fn handler_404() -> impl IntoResponse {
    build_response(StatusCode::NOT_FOUND, None::<String>, Some("Nothing to see here"))
}

#[derive(Serialize, Clone)]
pub struct UserCreatedResponse {
    id: i32,
}

async fn create_user(Json(payload): Json<Value>) -> impl IntoResponse {
    println!("{:?}", payload);
    let body = json!(UserCreatedResponse { id: 1 });
    build_response(StatusCode::CREATED, Some(body), None::<String>)
}

async fn update_user(Json(payload): Json<Value>) -> impl IntoResponse {
    println!("{:?}", payload);
    build_response(StatusCode::OK, Some("OK"), None::<String>)
}

#[derive(Deserialize, Clone, Debug)]
struct ReadUserQueryParams {
    email: String,
}

async fn read_user(Query(params): Query<ReadUserQueryParams>) -> impl IntoResponse {
    println!("{:?}", params);
    let body = json!({ "email": params.email });
    build_response(StatusCode::OK, Some(body), None::<String>)
}
