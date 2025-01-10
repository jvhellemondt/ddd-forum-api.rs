// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Int4,
        post_id -> Int4,
        member_id -> Int4,
        text -> Text,
        parent_comment_id -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    members (id) {
        id -> Int4,
        user_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        member_id -> Int4,
        post_type -> Varchar,
        title -> Varchar,
        content -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        username -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        password -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    votes (id) {
        id -> Int4,
        post_id -> Int4,
        member_id -> Int4,
        vote_type -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(comments -> members (member_id));
diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(members -> users (user_id));
diesel::joinable!(posts -> members (member_id));
diesel::joinable!(votes -> members (member_id));
diesel::joinable!(votes -> posts (post_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    members,
    posts,
    users,
    votes,
);
