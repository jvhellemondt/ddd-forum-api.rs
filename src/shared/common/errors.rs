use serde::Serialize;

#[derive(Serialize)]
pub enum CommonErrors {
    ValidationError,
    ServerError,
    ClientError,
    UnexpectedServerError,
}
