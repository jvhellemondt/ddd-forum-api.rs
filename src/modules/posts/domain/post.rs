use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::modules::posts::domain::comment::Comment;
use crate::modules::posts::domain::member_posted_by::MemberPostedBy;
use crate::modules::posts::domain::vote::Vote;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: i32,
    pub member_id: i32,
    pub post_type: String,
    pub title: String,
    pub content: String,
    pub votes: Vec<Vote>,
    pub member_posted_by: Option<MemberPostedBy>,
    pub comments: Vec<Comment>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
