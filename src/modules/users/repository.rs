use std::sync::Mutex;

use rusqlite::{Connection, named_params};

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
            "INSERT INTO users (\
                        email, username, first_name, last_name, password, created_at, updated_at\
                    ) VALUES (\
                        :email, :username, :first_name, :last_name, :password, :created_at, :updated_at\
                    )",
            named_params! {
                    ":email": user.email,
                    ":username": user.username,
                    ":first_name": user.first_name,
                    ":last_name": user.last_name,
                    ":password": user.password,
                    ":created_at": user.created_at,
                    ":updated_at": user.updated_at
                },
        ) {
            Ok(_) => Ok(conn.last_insert_rowid()),
            Err(err) => Err(err),
        }
    }

    fn get_by(&self, key: &str, value: &str) -> Result<Option<UserModel>, rusqlite::Error> {
        let conn = self.connection.lock().expect("Failed to lock the connection");

        let query = format!(
            "SELECT id, email, username, first_name, last_name, created_at, updated_at \
            FROM users \
            WHERE {} = :value",
            key
        );

        let mut stmt = conn.prepare(&query)?;

        let mut rows = stmt.query(named_params! { ":value": value })?;

        if let Some(row) = rows.next()? {
            let user = UserModel {
                id: row.get(0)?,
                email: row.get(1)?,
                username: row.get(2)?,
                first_name: row.get(3)?,
                last_name: row.get(4)?,
                password: None,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            };
            return Ok(Some(user));
        }
        Ok(None)
    }

    //
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
