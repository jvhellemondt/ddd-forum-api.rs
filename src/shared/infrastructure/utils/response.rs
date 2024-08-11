use axum::Json;
use axum::response::IntoResponse;
use http::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T, E> {
    pub data: Option<T>,
    pub error: Option<E>,
    pub success: bool,
}

pub fn build_response<T, E>(
    status: StatusCode,
    data: Option<T>,
    error: Option<E>,
) -> impl IntoResponse
where
    T: Serialize + Clone,
    E: Serialize + Clone,
{
    (status, Json(ApiResponse {
        data: data.clone(),
        error: error.clone(),
        success: error.is_none(),
    }))
}
