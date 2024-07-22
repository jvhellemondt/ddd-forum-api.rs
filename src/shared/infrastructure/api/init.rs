use std::time::Duration;

use axum;
use axum::{body::Bytes, extract::MatchedPath, http::{HeaderMap, Request}, response::Response, Router};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};

use crate::modules::common::infrastructure::api::routes as common;
use crate::modules::users::infrastructure::api::routes as users;

pub fn initialize_app() -> Router {
    let common_router = common::common_router();
    let users_router = users::users_router();

    let router = axum::Router::new()
        .nest("/", common_router)
        .nest("/", users_router)
        .layer(
        TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            let matched_path = request.extensions().get::<MatchedPath>().map(MatchedPath::as_str);
            info_span!("http",method = ?request.method(),matched_path,)
        }).on_request(|_request: &Request<_>, _span: &Span| {}).on_response(|response: &Response, latency: Duration, _span: &Span| {
            tracing::info!("Status {:?} | Latency {:?}", response.status(), latency);
        }).on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {}).on_eos(
            |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {},
        ).on_failure(
            |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {},
        ),
    );
    tracing::debug!("App: routes initialized");
    return router;
}
