use rusqlite::{Result};

pub trait Repository<T> {
    fn create(entity: &T) -> Result<()>;
    // fn read(&self, id: i32) -> Result<T>;
    // fn update(&self, entity: &T) -> Result<()>;
    // fn delete(&self, id: i32) -> Result<()>;
    // fn list(&self) -> Result<Vec<T>>;
}
