use chrono::{DateTime, Local, Utc};

use crate::shared::infrastructure::database as db;

fn make_connection() {
    let conn = db::connection::get_connection();
    let conn_lock = conn.lock().unwrap();
    let current_datetime: String = conn_lock
        .query_row("SELECT datetime('now');", (), |row| row.get(0))
        .expect("Database connection: Failed to get current datetime");
    let current_datetime_utc = DateTime::parse_from_rfc3339(&format!("{}Z", current_datetime))
        .expect("Failed to parse datetime")
        .with_timezone(&Utc);
    let local_datetime = current_datetime_utc.with_timezone(&Local);
    tracing::debug!("Database: Connection established on {}", local_datetime);
}

pub fn initialize_database() {
    make_connection();
    db::models::users::table::create_users_table();
}
