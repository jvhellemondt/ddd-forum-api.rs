use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    pub id: i32,
    pub post_id: i32,
    pub text: String,
    pub member_id: i32,
    pub parent_comment_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
