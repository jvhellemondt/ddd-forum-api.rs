use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use serde::Serialize;

use crate::modules::common::use_cases::health::controller;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T, E> {
    pub data: Option<T>,
    pub error: Option<E>,
    pub success: bool,
}

pub async fn get_health() -> impl IntoResponse {
    let health_status = controller::handle();
    let response = ApiResponse {
        data: Some(health_status),
        error: None::<()>,
        success: true,
    };
    (StatusCode::OK, Json(response))
}
