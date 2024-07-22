use crate::modules::users::domain::user::{User, Factory};
use crate::modules::users::use_cases::create_user::view;

pub fn create(payload: view::UserRequestBody) -> view::UserResponse {
    let user = User::create(
        &payload.email,
        &payload.username,
        &payload.first_name,
        &payload.last_name,
    );

    view::UserResponse {
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
        username: user.username,
    }
}
