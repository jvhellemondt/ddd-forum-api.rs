use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::members)]
pub struct Member {
    pub id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::members)]
pub struct NewMember {
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Clone)]
#[diesel(belongs_to(Member, foreign_key = member_id))]
#[diesel(table_name = crate::schema::posts)]
pub struct Post {
    pub id: i32,
    pub member_id: i32,
    pub post_type: String,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost<'a> {
    pub member_id: i32,
    pub title: &'a str,
    pub content: &'a str,
    pub post_type: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Clone)]
#[diesel(belongs_to(Post, foreign_key = post_id))]
#[diesel(belongs_to(Member, foreign_key = member_id))]
#[diesel(table_name = crate::schema::comments)]
pub struct Comment {
    pub id: i32,
    pub post_id: i32,
    pub member_id: i32,
    pub text: String,
    pub parent_comment_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::comments)]
pub struct NewComment<'a> {
    pub text: &'a str,
    pub member_id: i32,
    pub post_id: i32,
    pub parent_comment_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Clone)]
#[diesel(belongs_to(Post, foreign_key = post_id))]
#[diesel(belongs_to(Member, foreign_key = member_id))]
#[diesel(table_name = crate::schema::votes)]
pub struct Vote {
    pub id: i32,
    pub post_id: i32,
    pub member_id: i32,
    pub vote_type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::votes)]
pub struct NewVote<'a> {
    pub post_id: i32,
    pub vote_type: &'a str,
    pub member_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
