use crate::modules::users::use_cases::create_user;

pub fn handle(payload: create_user::view::UserCreateRequestBody) -> create_user::view::UserCreatedResponse {
    create_user::model::create(payload)
}
