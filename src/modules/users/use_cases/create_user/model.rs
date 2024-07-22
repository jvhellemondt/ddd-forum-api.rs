use chrono::{DateTime, Local};
use ulid::Ulid;

use crate::modules::users::repository::UserRepository;
use crate::modules::users::use_cases::create_user::view;
use crate::shared::infrastructure::database::repository::Repository;

pub struct UserModel {
    pub id: String,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

pub fn create(payload: view::UserCreateRequestBody) -> view::UserCreatedResponse {
    let now = Local::now();
    let user = UserModel {
        id: Ulid::new().to_string(),
        email: payload.email.to_string(),
        username: payload.username.to_string(),
        first_name: payload.first_name.to_string(),
        last_name: payload.last_name.to_string(),
        password: Ulid::new().to_string(),
        created_at: now,
        updated_at: now,
    };

    UserRepository::create(&user).expect("TODO: panic message");

    view::UserCreatedResponse {
        id: user.id
    }
}
