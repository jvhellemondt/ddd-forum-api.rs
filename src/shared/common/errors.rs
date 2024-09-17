use thiserror::Error;
use serde::Serialize;

#[derive(Serialize, Debug, Error)]
pub enum CommonErrors {
    #[error("Validation error occurred")]
    ValidationError,

    #[error("Server encountered an unexpected error")]
    ServerError,

    #[error("Database error")]
    DatabaseError,
}
