pub trait Repository<T, E> {
    fn create(&self, entity: &T) -> Result<(), E>;
    // fn read(&self, id: i32) -> Result<T, E>;
    // fn update(&self, entity: &T) -> Result<()>;
    // fn delete(&self, id: i32) -> Result<()>;
    // fn list(&self) -> Result<Vec<T>>;
}
