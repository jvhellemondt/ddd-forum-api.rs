use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;
use thiserror::Error;

use crate::modules::posts::domain::comment::Comment;
use crate::modules::posts::domain::member_posted_by::MemberPostedBy;
use crate::modules::posts::domain::post::Post;
use crate::modules::posts::domain::vote::Vote;
use crate::modules::posts::errors::PostsModuleErrors;
use crate::modules::posts::repositories::post_repository::PostRepository;
use crate::modules::users::domain::user::User;
use crate::shared::common::errors::CommonErrors;
use crate::shared::infrastructure::database::connection::get_db_pool;

#[derive(Debug, Error)]
pub enum ListPostErrors {
    #[error("Failed to fetch posts")]
    SqlSelectPostsFailed,
    #[error("Failed to fetch votes")]
    SqlSelectVotesFailed,
    #[error("Failed to fetch comments")]
    SqlSelectCommentsFailed,
    #[error("Failed to fetch users/members")]
    SqlSelectUsersMembersFailed,

}


pub struct PostgresPostRepository {
    pub pool: Arc<PgPool>,
}

impl PostgresPostRepository {
    pub fn new() -> Self {
        let pool = get_db_pool().clone();
        PostgresPostRepository { pool }
    }
}

#[async_trait]
impl PostRepository for PostgresPostRepository {
    async fn list_posts(&self, _sort: Option<String>) -> Result<Vec<Post>, PostsModuleErrors> {
        let posts_result = sqlx::query!(
            r#"
            SELECT
                id,
                member_id,
                title,
                content,
                post_type,
                created_at,
                updated_at
            FROM posts
            ORDER BY created_at DESC
            "#
        ).fetch_all(&*self.pool).await;

        let posts = match posts_result {
            Ok(posts) => posts,
            Err(_) => {
                return Err(PostsModuleErrors::CommonError(CommonErrors::DatabaseError(Some(ListPostErrors::SqlSelectPostsFailed.to_string()))));
            }
        };

        let post_ids: Vec<i32> = posts.iter().map(|post| post.id).collect();
        let member_ids: Vec<i32> = posts.iter().map(|post| post.member_id).collect();

        let votes_result = sqlx::query!(
            r#"
            SELECT
                id,
                post_id,
                member_id,
                vote_type,
                created_at,
                updated_at
            FROM votes
            WHERE post_id = ANY($1)
            "#,
            &post_ids
        ).fetch_all(&*self.pool).await;

        let votes = match votes_result {
            Ok(votes) => votes,
            Err(_) => {
                return Err(PostsModuleErrors::CommonError(CommonErrors::DatabaseError(Some(ListPostErrors::SqlSelectVotesFailed.to_string()))));
            }
        };

        let mut votes_map: HashMap<i32, Vec<Vote>> = HashMap::new();
        for row in votes {
            votes_map.entry(row.post_id).or_insert_with(Vec::new).push(Vote {
                id: row.id,
                post_id: row.post_id,
                member_id: row.member_id,
                vote_type: row.vote_type,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }

        let comments_result = sqlx::query!(
            r#"
            SELECT
                id,
                post_id,
                text,
                member_id,
                parent_comment_id,
                created_at,
                updated_at
            FROM comments
            WHERE post_id = ANY($1)
            "#,
            &post_ids
        ).fetch_all(&*self.pool).await;

        let comments = match comments_result {
            Ok(comments) => comments,
            Err(_) => {
                return Err(PostsModuleErrors::CommonError(CommonErrors::DatabaseError(Some(ListPostErrors::SqlSelectCommentsFailed.to_string()))));
            }
        };

        let mut comments_map: HashMap<i32, Vec<Comment>> = HashMap::new();
        for row in comments {
            comments_map.entry(row.post_id).or_insert_with(Vec::new).push(Comment {
                id: row.id,
                post_id: row.post_id,
                text: row.text,
                member_id: row.member_id,
                parent_comment_id: row.parent_comment_id,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }

        let members_and_users_result = sqlx::query!(
            r#"
            SELECT
                members.id AS member_id,
                members.user_id,
                members.created_at AS member_created_at,
                members.updated_at AS member_updated_at,
                users.email,
                users.first_name,
                users.last_name,
                users.username,
                users.password,
                users.created_at,
                users.updated_at
            FROM members
            JOIN users ON members.user_id = users.id
            WHERE members.id = ANY($1)
            "#,
            &member_ids
        ).fetch_all(&*self.pool).await;

        let members_and_users = match members_and_users_result {
            Ok(members_and_users) => members_and_users,
            Err(_) => {
                return Err(PostsModuleErrors::CommonError(CommonErrors::DatabaseError(Some(ListPostErrors::SqlSelectUsersMembersFailed.to_string()))));
            }
        };

        let mut members_map: HashMap<i32, MemberPostedBy> = HashMap::new();
        for row in members_and_users {
            members_map.insert(
                row.member_id,
                MemberPostedBy {
                    id: row.member_id,
                    user_id: row.user_id,
                    user: User {
                        id: row.user_id,
                        email: row.email,
                        first_name: row.first_name,
                        last_name: row.last_name,
                        username: row.username,
                        password: None,
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                    },
                },
            );
        }

        let posts_with_data: Vec<Post> = posts.into_iter().map(|post| {
            Post {
                id: post.id,
                member_id: post.member_id,
                post_type: post.post_type,
                title: post.title,
                content: post.content,
                created_at: post.created_at,
                updated_at: post.updated_at,
                votes: votes_map.get(&post.id).cloned().unwrap_or_default(),
                comments: comments_map.get(&post.id).cloned().unwrap_or_default(),
                member_posted_by: members_map.get(&post.member_id).cloned(),
            }
        }).collect();

        Ok(posts_with_data)

        // Err(_) => {
        //     Err(PostsModuleErrors::CommonError(CommonErrors::DatabaseError))
        // }
    }
}
