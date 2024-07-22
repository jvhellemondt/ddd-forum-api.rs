use crate::modules::users::use_cases::create_user;

pub fn handle(payload: create_user::view::UserRequestBody) -> create_user::view::UserResponse {
    create_user::model::create(payload)
}
