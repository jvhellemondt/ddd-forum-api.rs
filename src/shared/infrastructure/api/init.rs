use std::time::Duration;

use axum::{
    self,
    extract::MatchedPath,
    http::{Request},
    response::Response,
    Router,
};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tower_http::cors::CorsLayer;
use tracing::{info_span, Span};

use crate::modules::common::infrastructure::api::routes::common_router;
use crate::modules::users::infrastructure::api::routes::users_router;
use crate::modules::posts::infrastructure::api::routes::posts_router;

pub fn initialize_app() -> Router {
    let api_routes = Router::new()
        .nest("/", common_router())
        .nest("/", users_router())
        .nest("/", posts_router());

    let router = axum::Router::new()
        .nest("/api", api_routes)
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
    tracing::info!("Api: routes initialized");
    return router;
}
