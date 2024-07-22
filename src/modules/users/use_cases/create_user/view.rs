use axum::{Json, response::IntoResponse};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::modules::users::use_cases::create_user::controller;
use crate::shared::infrastructure::utils::api_response;
use crate::shared::infrastructure::utils::api_response::ApiResponse;

#[derive(Deserialize)]
pub struct UserCreateRequestBody {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize)]
pub struct UserCreatedResponse {
    pub id: String,
}

pub async fn post_create_user(Json(payload): Json<UserCreateRequestBody>) -> impl IntoResponse {
    let user_created = controller::handle(payload);
    api_response::build(
        ApiResponse {
            success: true,
            data: Some(user_created),
            error: None::<()>,
        },
        StatusCode::CREATED
    )
}
