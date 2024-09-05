pub mod domain {
    pub mod comment;
    pub mod post;
    pub mod vote;
    pub mod member_posted_by;
}
pub mod use_cases {
    pub mod list_posts {
        pub mod view;
        pub mod model;
        pub mod controller;
    }
}

pub mod repositories {
    pub mod post_repository;

    pub mod implementations {
        pub mod postgres_post_repository;
    }
}

pub mod infrastructure {
    pub mod api {
        pub mod routes;
    }
}

mod errors;
