// @TODO: refactor not to use rusqlite here :-(
use rusqlite::ToSql;

pub trait Repository<T, E> {
    fn create(&self, entity: &T) -> Result<i64, E>;
    fn get_by<U: ToSql>(&self, key: &str, value: U) -> Result<Option<T>, E>;
    // fn read(&self, id: i32) -> Result<T, E>;
    fn update(&self, entity: &T) -> Result<(), E>;
    // fn delete(&self, id: i32) -> Result<()>;
    // fn list(&self) -> Result<Vec<T>>;
}
