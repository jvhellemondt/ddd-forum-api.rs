use axum::{Json, response::IntoResponse};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::modules::users::use_cases::create_user::controller;
use crate::shared::common::errors::CommonErrors;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T, E> {
    pub data: Option<T>,
    pub error: Option<E>,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreateRequestBody {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize)]
pub struct UserCreatedResponse {
    pub id: i64,
}

pub async fn post_create_user(Json(payload): Json<Value>) -> impl IntoResponse {
    match controller::handle(payload) {
        Ok(user) => {
            let response = ApiResponse {
                data: Some(UserCreatedResponse { id: user.id }),
                error: None::<CommonErrors>,
                success: true,
            };
            (StatusCode::CREATED, Json(response))
        }
        Err(CommonErrors::ValidationError) => {
            let response = ApiResponse {
                data: None::<UserCreatedResponse>,
                error: Some(CommonErrors::ValidationError),
                success: false,
            };
            (StatusCode::CONFLICT, Json(response))
        }
        _ => {
            let response = ApiResponse {
                data: None::<UserCreatedResponse>,
                error: Some(CommonErrors::UnexpectedServerError),
                success: false,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}
