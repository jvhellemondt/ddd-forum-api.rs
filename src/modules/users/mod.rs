pub mod domain {
    pub mod user_entity;
}

pub mod use_cases {
    pub mod create_user {
        pub mod controller;
        pub mod model;
        pub mod view;
    }

    pub mod get_user_by_email {
        pub mod controller;
        pub mod model;
        pub mod view;
    }

    pub mod update_user {
        pub mod controller;
        pub mod model;
        pub mod view;
    }
}

pub mod errors;
pub mod repositories {
    pub mod user_repository;

    pub mod implementations {
        pub mod postgres_user_repository;
    }
}

pub mod infrastructure {
    pub mod api {
        pub mod routes;
    }
}
