use std::sync::Mutex;

use rusqlite::{Connection, params};

use crate::modules::users::use_cases::create_user::model::UserModel;
use crate::shared::infrastructure::database::repository::Repository;

pub struct UsersRepository {
    connection: &'static Mutex<Connection>,
}

impl UsersRepository {
    pub fn new(connection: &'static Mutex<Connection>) -> Self {
        UsersRepository { connection }
    }
}

impl Repository<UserModel, rusqlite::Error> for UsersRepository {
    fn create(&self, user: &UserModel) -> Result<i64, rusqlite::Error> {
        let conn = self.connection.lock().expect("Failed to lock the connection");
        match conn.execute(
            "INSERT INTO users (email, username, first_name, last_name, password, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                    user.email,
                    user.username,
                    user.first_name,
                    user.last_name,
                    user.password,
                    user.created_at.to_rfc3339(),
                    user.updated_at.to_rfc3339()
                ],
        ) {
            Ok(_) => Ok(conn.last_insert_rowid()),
            Err(err) => Err(err),
        }
    }

    // fn read(&self, id: &str) -> Result<User, ()> {
    //     let conn = self.connection.lock().expect("Failed to lock the connection");
    //
    //     let mut stmt = conn.prepare("SELECT id, email, username, first_name, last_name, password, created_at, updated_at FROM users WHERE id = ?1")?;
    //     let user = stmt.query_row(params![id], |row| {
    //         Ok(User {
    //             id: row.get(0)?,
    //             email: row.get(1)?,
    //             username: row.get(2)?,
    //             first_name: row.get(3)?,
    //             last_name: row.get(4)?,
    //             password: row.get(5)?,
    //             created_at: Local.timestamp(row.get(6)?, 0),
    //             updated_at: Local.timestamp(row.get(7)?, 0),
    //         })
    //     })?;
    //     Ok(user)
    // }
    //
    // fn update(&self, user: &User) -> Result<(), ()> {
    //     let conn = self.connection.lock().expect("Failed to lock the connection");
    //
    //     let sql = "UPDATE users SET email = ?1, username = ?2, first_name = ?3, last_name = ?4, password = ?5, created_at = ?6, updated_at = ?7 WHERE id = ?8";
    //     conn.execute(
    //         sql,
    //         params![
    //             user.email,
    //             user.username,
    //             user.first_name,
    //             user.last_name,
    //             user.password,
    //             user.created_at.timestamp(),
    //             user.updated_at.timestamp(),
    //             user.id
    //         ],
    //     )?;
    //     Ok(())
    // }
    //
    // fn delete(&self, id: &str) -> Result<(), ()> {
    //     let conn = self.connection.lock().expect("Failed to lock the connection");
    //
    //     conn.execute("DELETE FROM users WHERE id = ?1", params![id])?;
    //     Ok(())
    // }
    //
    // fn list(&self) -> Result<Vec<User>, ()> {
    //     let conn = self.connection.lock().expect("Failed to lock the connection");
    //
    //     let mut stmt = conn.prepare("SELECT id, email, username, first_name, last_name, password, created_at, updated_at FROM users")?;
    //     let user_iter = stmt.query_map((), |row| {
    //         Ok(User {
    //             id: row.get(0)?,
    //             email: row.get(1)?,
    //             username: row.get(2)?,
    //             first_name: row.get(3)?,
    //             last_name: row.get(4)?,
    //             password: row.get(5)?,
    //             created_at: Local.timestamp(row.get(6)?, 0),
    //             updated_at: Local.timestamp(row.get(7)?, 0),
    //         })
    //     })?;
    //
    //     let mut users = Vec::new();
    //     for user in user_iter {
    //         users.push(user?);
    //     }
    //     Ok(users)
    // }
}
