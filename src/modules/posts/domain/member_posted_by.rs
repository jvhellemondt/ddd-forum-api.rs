use serde::{Deserialize, Serialize};
use crate::modules::users::domain::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemberPostedBy {
    pub id: i32,
    pub user_id: i32,
    pub user: User,
}
