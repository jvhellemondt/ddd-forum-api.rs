use axum::{Json, response::IntoResponse};
use http::StatusCode;
use serde::Serialize;
use serde_json::Value;

use crate::modules::users::errors::{UsersDomainErrors, UsersModuleErrors};
use crate::modules::users::use_cases::create_user::controller;
use crate::shared::common::errors::CommonErrors;
use crate::shared::infrastructure::utils::response::build_response;

#[derive(Serialize, Clone)]
pub struct UserCreatedResponse {
    pub id: i32,
}

pub async fn post_create_user(Json(payload): Json<Value>) -> impl IntoResponse {
    match controller::handle(payload).await {
        Ok(id) => build_response(
            StatusCode::CREATED,
            Some(UserCreatedResponse { id }),
            None,
        ),
        Err(UsersModuleErrors::DomainError(UsersDomainErrors::EmailAlreadyInUse)) => build_response(
            StatusCode::CONFLICT,
            None,
            Some(UsersDomainErrors::EmailAlreadyInUse.to_string()),
        ),
        Err(UsersModuleErrors::DomainError(UsersDomainErrors::UsernameAlreadyTaken)) => build_response(
            StatusCode::CONFLICT,
            None,
            Some(UsersDomainErrors::UsernameAlreadyTaken.to_string()),
        ),
        Err(UsersModuleErrors::DomainError(UsersDomainErrors::UserNotFound)) => build_response(
            StatusCode::NOT_FOUND,
            None,
            Some(UsersDomainErrors::UserNotFound.to_string()),
        ),
        Err(UsersModuleErrors::CommonError(CommonErrors::ValidationError)) => build_response(
            StatusCode::BAD_REQUEST,
            None,
            Some(CommonErrors::ValidationError.to_string()),
        ),
        Err(UsersModuleErrors::CommonError(CommonErrors::ServerError)) => build_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some(CommonErrors::ServerError.to_string()),
        ),
        Err(UsersModuleErrors::CommonError(CommonErrors::DatabaseError)) => build_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some(CommonErrors::DatabaseError.to_string()),
        ),
    }
}
