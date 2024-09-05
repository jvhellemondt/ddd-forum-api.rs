use axum::{Router, routing::get};

use crate::modules::posts::use_cases::list_posts::view::list_posts;

pub fn posts_router() -> Router {
    Router::new()
        .route("/posts", get(list_posts))
}
