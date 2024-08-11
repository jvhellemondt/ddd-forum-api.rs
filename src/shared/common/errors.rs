use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum CommonErrors {
    ValidationError,
    ServerError,
    ClientError,
    UnexpectedServerError,
}
