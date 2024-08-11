use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UserModel {
    pub id: Option<i64>,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
