pub trait Repository<T, E> {
    fn create(&self, entity: &T) -> Result<i64, E>;
    fn get_by(&self, key: &str, value: &str) -> Result<Option<T>, E>;
    // fn read(&self, id: i32) -> Result<T, E>;
    // fn update(&self, entity: &T) -> Result<()>;
    // fn delete(&self, id: i32) -> Result<()>;
    // fn list(&self) -> Result<Vec<T>>;
}