use chrono::{DateTime, Local};
use ulid::Ulid;

use crate::modules::users::{repository::UsersRepository, use_cases::create_user::view};
use crate::shared::common::errors::CommonErrors;
use crate::shared::infrastructure::database::repository::Repository;
use crate::shared::infrastructure::database as db;

pub struct UserModel {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

pub fn create(payload: view::UserCreateRequestBody) -> Result<view::UserCreatedResponse, CommonErrors> {
    let now = Local::now();
    let user = UserModel {
        email: payload.email.to_string(),
        username: payload.username.to_string(),
        first_name: payload.first_name.to_string(),
        last_name: payload.last_name.to_string(),
        password: Ulid::new().to_string(),
        created_at: now,
        updated_at: now,
    };

    let repository = UsersRepository::new(db::connection::get_connection());
    println!("@here");
    match repository.create(&user) {
        Ok(id) => Ok(view::UserCreatedResponse { id }),
        Err(e) => {
            println!("{:?}", e);
            Err(CommonErrors::UnexpectedServerError)
        },
        _ => Err(CommonErrors::UnexpectedServerError)
    }
}
