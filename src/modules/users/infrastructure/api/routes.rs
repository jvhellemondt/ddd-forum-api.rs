use axum::{routing::post, Router};

use crate::modules::users::use_cases::create_user::view::post_create_user;

pub fn users_router() -> Router {
    return Router::new().route("/users/new", post(post_create_user));
}
