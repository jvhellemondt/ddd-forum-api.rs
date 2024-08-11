use axum::{Router, routing::{get, post}};

use crate::modules::users::use_cases::create_user::view::post_create_user;
use crate::modules::users::use_cases::get_user_by_email::view::get_user_by_email;

pub fn users_router() -> Router {
    Router::new()
        .route("/users/new", post(post_create_user))
        .route("/users", get(get_user_by_email))
}
