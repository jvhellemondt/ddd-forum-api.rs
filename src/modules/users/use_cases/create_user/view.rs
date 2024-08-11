use axum::{Json, response::IntoResponse};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::modules::users::errors::{UsersDomainErrors, UsersErrors};
use crate::modules::users::use_cases::create_user::controller;
use crate::shared::common::errors::CommonErrors;
use crate::shared::infrastructure::utils::response::build_response;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreateRequestBody {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserCreatedResponse {
    pub id: i64,
}

pub async fn post_create_user(Json(payload): Json<Value>) -> impl IntoResponse {
    match controller::handle(payload).await {
        Ok(id) => build_response(
            StatusCode::CREATED,
            Some(UserCreatedResponse { id }),
            None,
        ),
        Err(UsersErrors::DomainError(UsersDomainErrors::EmailAlreadyInUse)) => build_response(
            StatusCode::CONFLICT,
            None,
            Some("EmailAlreadyInUse".to_string()),
        ),
        Err(UsersErrors::DomainError(UsersDomainErrors::UsernameAlreadyTaken)) => build_response(
            StatusCode::CONFLICT,
            None,
            Some("UsernameAlreadyTaken".to_string()),
        ),
        Err(UsersErrors::CommonError(CommonErrors::ValidationError)) => build_response(
            StatusCode::BAD_REQUEST,
            None,
            Some("ValidationError".to_string()),
        ),
        Err(UsersErrors::CommonError(CommonErrors::UnexpectedServerError)) => build_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some("ServerError".to_string()),
        ),
        _ => build_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some("ServerError".to_string()),
        )
    }
}
