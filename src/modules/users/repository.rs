use std::sync::Mutex;

use chrono::Local;
use rusqlite::{Connection, named_params, ToSql};

use crate::modules::users::domain::user::UserModel;
use crate::modules::users::errors::UsersErrors::{self, CommonError};
use crate::shared::common::errors::CommonErrors;
use crate::shared::infrastructure::database::repository::Repository;

pub struct UsersRepository {
    connection: &'static Mutex<Connection>,
}

impl UsersRepository {
    pub fn new(connection: &'static Mutex<Connection>) -> Self {
        UsersRepository { connection }
    }
}

impl Repository<UserModel, UsersErrors> for UsersRepository {
    fn create(&self, user: &UserModel) -> Result<i64, UsersErrors> {
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
            Err(_) => Err(CommonError(CommonErrors::ServerError)),
        }
    }

    fn get_by<T: ToSql>(&self, key: &str, value: T) -> Result<Option<UserModel>, UsersErrors> {
        let conn = self.connection.lock().expect("Failed to lock the connection");

        let query = format!(
            "SELECT id, email, username, first_name, last_name, password, created_at, updated_at \
             FROM users WHERE {} = :value",
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
                password: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            };
            return Ok(Some(user));
        }
        Ok(None)
    }

    fn update(&self, user: &UserModel) -> Result<(), UsersErrors> {
        let conn = self.connection.lock().expect("Failed to lock the connection");
        match conn.execute(
            "UPDATE users SET username = :username, first_name = :first_name, last_name = :last_name, email = :email, updated_at = :updated_at WHERE id = :id",
            named_params! {
                ":username": user.username,
                ":first_name": user.first_name,
                ":last_name": user.last_name,
                ":email": user.email,
                ":updated_at": Local::now().to_rfc3339(),
                ":id": user.id,
           },
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err(CommonError(CommonErrors::ServerError)),
        }
    }
}
