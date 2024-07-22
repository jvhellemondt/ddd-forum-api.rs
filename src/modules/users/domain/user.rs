use chrono::{DateTime, Local};
use ulid::Ulid;

#[derive(Debug)]
pub struct User {
    id: String,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    password: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

pub trait Factory {
    fn create(
        email: &String,
        username: &String,
        first_name: &String,
        last_name: &String,
    ) -> User;
}

impl Factory for User {
    fn create(
        email: &String,
        username: &String,
        first_name: &String,
        last_name: &String,
    ) -> User {
        let now = Local::now();
        User {
            id: Ulid::new().to_string(),
            email: email.to_string(),
            username: username.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            password: Ulid::new().to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}
