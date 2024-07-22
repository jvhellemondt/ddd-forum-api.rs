use axum::{Router, routing::get};

use crate::modules::common::use_cases::health::view::get_health;

pub fn common_router() -> Router {
    Router::new().route("/health", get(get_health))
}
