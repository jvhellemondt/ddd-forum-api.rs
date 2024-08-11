pub mod domain {
    pub mod user;
}

pub mod use_cases {
    pub mod create_user {
        pub mod controller;
        pub mod model;
        pub mod view;
        pub mod errors;
    }
}

pub mod errors;
pub mod repository;

pub mod infrastructure {
    pub mod api {
        pub mod routes;
    }
}
