use axum::extract::Query;
use axum::response::IntoResponse;
use http::StatusCode;
use serde::Deserialize;

use crate::modules::users::repository::UsersRepository;
use crate::shared::infrastructure::database as db;
use crate::shared::infrastructure::database::repository::Repository;
use crate::shared::infrastructure::utils::response::build_response;

#[derive(Deserialize)]
pub struct GetUserByEmailParams {
    email: String,
}

pub async fn get_user_by_email(Query(params): Query<GetUserByEmailParams>) -> impl IntoResponse {
    let repository = UsersRepository::new(db::connection::get_connection());
    let email = params.email;

    match repository.get_by("email", &email) {
        Ok(Some(user)) => build_response(
            StatusCode::OK,
            Some(user),
            None,
        ),
        _ => build_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some("ServerError".to_string()),
        )
    }

    // match controller::handle(payload).await {
    //     Ok(id) => build_response(
    //         StatusCode::OK,
    //         Some(Response { id }),
    //         None,
    //     ),
    //     _ => build_response(
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         None,
    //         Some("ServerError".to_string()),
    //     )
    // }
}
