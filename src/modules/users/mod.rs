mod repository;
mod r#errors

pub mod infrastructure {
    pub mod api {
        pub mod routes;
    }
}

pub mod use_cases {
    pub mod create_user {
        pub mod controller;
        pub mod view;
        pub mod model;
    }
}
