use axum::{Json, response::IntoResponse};
use axum::extract::Path;
use http::StatusCode;
use serde_json::Value;

use crate::modules::users::errors::{UsersDomainErrors, UsersModuleErrors};
use crate::modules::users::errors::UsersModuleErrors::CommonError;
use crate::modules::users::use_cases::update_user::controller;
use crate::shared::common::errors::CommonErrors;
use crate::shared::infrastructure::utils::response::build_response;

pub async fn update_user(Path(id): Path<i32>, Json(payload): Json<Value>) -> impl IntoResponse {
    match controller::handle(payload, id).await {
        Ok(_) => build_response(
            StatusCode::OK,
            Some("OK".to_string()),
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
        Err(CommonError(CommonErrors::ValidationError)) => build_response(
            StatusCode::BAD_REQUEST,
            None,
            Some(CommonError(CommonErrors::ValidationError).to_string()),
        ),
        _ => build_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some(CommonError(CommonErrors::ServerError).to_string()),
        )
    }
}
