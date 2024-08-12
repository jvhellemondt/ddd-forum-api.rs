use axum::{Router, routing::{get, post, put}};

use crate::modules::users::use_cases::create_user::view::post_create_user;
use crate::modules::users::use_cases::get_user_by_email::view::get_user_by_email;
use crate::modules::users::use_cases::update_user::view::update_user;

pub fn users_router() -> Router {
    Router::new()
        .route("/users/create", post(post_create_user))
        .route("/users/update/:id", put(update_user))
        .route("/users", get(get_user_by_email))
}
