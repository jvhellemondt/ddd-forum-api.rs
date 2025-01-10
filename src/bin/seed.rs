use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use ddd_forum_api::models::{NewUser, NewMember, NewPost, NewVote, NewComment};
use ddd_forum_api::schema::{users, members, posts, votes, comments};
use chrono::{Utc};
use diesel::insert_into;

fn seed_data(conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
    let now = Utc::now().naive_utc();

    let new_users = vec![
        NewUser {
            email: "bobvance@gmail.com".to_string(),
            first_name: Some("Bob".to_string()),
            last_name: Some("Vance".to_string()),
            username: "bobvance".to_string(),
            password: "123".to_string(),
            created_at: now,
            updated_at: now,
        },
        NewUser {
            email: "tonysoprano@gmail.com".to_string(),
            first_name: Some("Tony".to_string()),
            last_name: Some("Soprano".to_string()),
            username: "tonysoprano".to_string(),
            password: "123".to_string(),
            created_at: now,
            updated_at: now,
        },
        NewUser {
            email: "billburr@gmail.com".to_string(),
            first_name: Some("Bill".to_string()),
            last_name: Some("Burr".to_string()),
            username: "billburr".to_string(),
            password: "123".to_string(),
            created_at: now,
            updated_at: now,
        },
    ];

    insert_into(users::table)
        .values(&new_users)
        .execute(conn)?;

    let new_members = vec![
        NewMember {
            user_id: 1,
            created_at: now,
            updated_at: now,
        },
        NewMember {
            user_id: 2,
            created_at: now,
            updated_at: now,
        },
        NewMember {
            user_id: 3,
            created_at: now,
            updated_at: now,
        },
    ];

    insert_into(members::table)
        .values(&new_members)
        .execute(conn)?;

    let new_posts = vec![
        NewPost {
            member_id: 1,
            title: "First post!",
            content: "This is Bob Vance's first post",
            post_type: "Text",
            created_at: now,
            updated_at: now,
        },
        NewPost {
            member_id: 1,
            title: "Second post!",
            content: "This is Bob's second post",
            post_type: "Text",
            created_at: now,
            updated_at: now,
        },
        NewPost {
            member_id: 2,
            title: "Another post",
            content: "This is Tony's first post",
            post_type: "Text",
            created_at: now,
            updated_at: now,
        },
        NewPost {
            member_id: 2,
            title: "Links",
            content: "This is a link post",
            post_type: "Link",
            created_at: now,
            updated_at: now,
        },
    ];

    insert_into(posts::table)
        .values(&new_posts)
        .execute(conn)?;

    let new_votes = vec![
        NewVote {
            post_id: 1,
            vote_type: "Upvote",
            member_id: 1,
            created_at: now,
            updated_at: now,
        },
        NewVote {
            post_id: 2,
            vote_type: "Upvote",
            member_id: 1,
            created_at: now,
            updated_at: now,
        },
        NewVote {
            post_id: 3,
            vote_type: "Upvote",
            member_id: 2,
            created_at: now,
            updated_at: now,
        },
        NewVote {
            post_id: 4,
            vote_type: "Upvote",
            member_id: 2,
            created_at: now,
            updated_at: now,
        },
        NewVote {
            post_id: 3,
            vote_type: "Upvote",
            member_id: 1,
            created_at: now,
            updated_at: now,
        },
        NewVote {
            post_id: 2,
            vote_type: "Downvote",
            member_id: 3,
            created_at: now,
            updated_at: now,
        },
    ];

    insert_into(votes::table)
        .values(&new_votes)
        .execute(conn)?;

    let new_comments = vec![
        NewComment {
            text: "I posted this!",
            member_id: 1,
            post_id: 1,
            parent_comment_id: None,
            created_at: now,
            updated_at: now,
        },
        NewComment {
            text: "Nice",
            member_id: 2,
            post_id: 2,
            parent_comment_id: None,
            created_at: now,
            updated_at: now,
        },
    ];

    insert_into(comments::table)
        .values(&new_comments)
        .execute(conn)?;

    Ok(())
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        _ => {
            panic!("DATABASE_URL must be set");
        }
    };

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let conn = &mut establish_connection();
    match seed_data(conn) {
        Ok(_) => println!("[Database] Seeding successful!"),
        Err(err) => println!("[Database] Failed to seed data: {:?}", err),
    }
}
