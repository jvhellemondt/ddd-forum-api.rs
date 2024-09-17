use std::env::VarError;
use thiserror::Error;
use sqlx::{Error as SqlxError};

#[derive(Debug, Error)]
pub enum DatabaseConnectionError {
    #[error("Environment variable not set: {0}")]
    EnvironmentVariableNotSet(#[from] VarError),

    #[error("Could not connect to the database: {0}")]
    CouldNotConnectError(#[from] SqlxError),

    #[error("Failed to initialize database pool")]
    PoolInitializationError,
}
