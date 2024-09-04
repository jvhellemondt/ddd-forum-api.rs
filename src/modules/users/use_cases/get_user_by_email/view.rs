use axum::extract::Query;
use axum::response::IntoResponse;
use http::StatusCode;
use serde::Deserialize;

use crate::modules::users::errors::UsersDomainErrors;
use crate::modules::users::errors::UsersModuleErrors::CommonError;
use crate::modules::users::use_cases::get_user_by_email::controller;
use crate::shared::common::errors::CommonErrors::ServerError;
use crate::shared::infrastructure::utils::response::build_response;

#[derive(Deserialize, Clone)]
pub struct GetUserByEmailParams {
    pub(crate) email: String,
}

pub async fn get_user_by_email(Query(params): Query<GetUserByEmailParams>) -> impl IntoResponse {
    let dto = GetUserByEmailParams {
        email: params.email
    };

    match controller::handle(dto).await {
        Ok(Some(user)) => build_response(
            StatusCode::OK,
            Some(user),
            None,
        ),
        Ok(None) => build_response(
            StatusCode::NOT_FOUND,
            None,
            Some(UsersDomainErrors::UserNotFound.to_string()),
        ),
        _ => build_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some(CommonError(ServerError).to_string()),
        )
    }
}
