use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T, E> {
    pub data: Option<T>,
    pub error: Option<E>,
    pub success: bool,
}

pub fn build<E, T>(
    result: ApiResponse<T, E>,
    code: StatusCode,
) -> impl IntoResponse where
    T: Serialize,
    E: Serialize,
{
    let mut response = (code, Json(result)).into_response();
    *response.status_mut() = code;
    response
}

pub fn build_error<E>(
    code: StatusCode,
    error: E,
) -> impl IntoResponse where
    E: Serialize,
{
    let response: ApiResponse<(), E> = ApiResponse {
        data: None::<()>,
        error: Some(error),
        success: false,
    };
    build(response, code)
}
