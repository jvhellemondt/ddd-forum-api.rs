use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UserEntity {
    pub id: Option<i32>,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
