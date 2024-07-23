use crate::shared::infrastructure::database as db;

const CREATE_USERS_TABLE_QUERY: &str = "
    CREATE TABLE IF NOT EXISTS users (
        id TEXT PRIMARY KEY,
        email TEXT NOT NULL UNIQUE,
        username TEXT NOT NULL UNIQUE,
        first_name TEXT NOT NULL,
        last_name TEXT NOT NULL,
        password TEXT NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL
      );
  ";

pub fn create_users_table() {
    let connection = db::connection::get_connection();
    let connection_lock = connection.lock().expect("Failed to lock the connection");
    connection_lock
        .execute(CREATE_USERS_TABLE_QUERY, ())
        .expect("Failed to create the users table");
    tracing::debug!("Database: Users table created");
}
