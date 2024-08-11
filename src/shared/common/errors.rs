use std::fmt;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum CommonErrors {
    ValidationError,
    ServerError,
}

impl fmt::Display for CommonErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant_str = match self {
            CommonErrors::ValidationError => "ValidationError",
            CommonErrors::ServerError => "UnexpectedServerError",
        };
        write!(f, "{}", variant_str)
    }
}
