use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vote {
    pub id: i32,
    pub post_id: i32,
    pub member_id: i32,
    pub vote_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
