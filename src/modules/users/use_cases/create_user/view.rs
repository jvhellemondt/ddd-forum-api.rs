use axum::{Json, response::IntoResponse};
use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::modules::users::use_cases::create_user::controller;

#[derive(Deserialize)]
pub struct UserRequestBody {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
}

pub async fn post_create_user(Json(body): Json<UserRequestBody>) -> impl IntoResponse {
    let user_created = controller::handle(body);
    Json(json!(user_created))
}
