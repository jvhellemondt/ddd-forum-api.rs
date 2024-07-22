pub mod database {
    pub mod connection;
    pub mod init;
    pub mod repository;

    pub mod models {
        pub mod users {
            pub mod table;
        }
    }
}

pub mod api {
    pub mod init;
}
