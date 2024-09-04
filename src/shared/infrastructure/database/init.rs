use crate::shared::infrastructure::database::connection::{get_db_pool, init_db_pool};
use anyhow::Result;

async fn check_connection() -> Result<()> {
    let pool = get_db_pool();
    sqlx::query("SELECT 1")
        .execute(&*pool)
        .await?;
    Ok(())
}

pub async fn execute() -> Result<()> {
    if let Err(e) = init_db_pool().await {
        eprintln!("Failed to initialize database: {}", e);
        return Err(e.into());
    }

    if let Err(e) = check_connection().await {
        eprintln!("Connection query failed: {}", e);
        return Err(e.into());
    }
    Ok(())
}
