use std::env;
use std::sync::Arc;
use sqlx::PgPool;
use once_cell::sync::OnceCell;

use crate::shared::infrastructure::database::errors::DatabaseConnectionError;

static DB_POOL: OnceCell<Arc<PgPool>> = OnceCell::new();

pub async fn create_pool() -> Result<Arc<PgPool>, DatabaseConnectionError> {
    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;
    Ok(Arc::new(pool))
}

pub async fn init_db_pool() -> Result<(), DatabaseConnectionError> {
    let pool = create_pool().await?;
    DB_POOL.set(pool).map_err(|_| DatabaseConnectionError::PoolInitializationError)?;
    Ok(())
}

pub fn get_db_pool() -> Arc<PgPool> {
    DB_POOL.get().expect("DB pool is not initialized").clone()
}
