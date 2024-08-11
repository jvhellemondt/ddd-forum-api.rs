use std::fmt;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum CommonErrors {
    ValidationError,
    UnexpectedServerError,
}

impl fmt::Display for CommonErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant_str = match self {
            CommonErrors::ValidationError => "ValidationError",
            CommonErrors::UnexpectedServerError => "UnexpectedServerError",
        };
        write!(f, "{}", variant_str)
    }
}
