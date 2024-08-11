use std::fmt;
use crate::shared::common::errors::CommonErrors;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum UsersDomainErrors {
    UsernameAlreadyTaken,
    EmailAlreadyInUse,
    UserNotFound,
}

#[derive(Debug)]
pub enum UsersErrors {
    DomainError(UsersDomainErrors),
    CommonError(CommonErrors),
    DatabaseError(()),
}

impl fmt::Display for UsersDomainErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant_str = match self {
            UsersDomainErrors::UsernameAlreadyTaken => "UsernameAlreadyTaken",
            UsersDomainErrors::EmailAlreadyInUse => "EmailAlreadyInUse",
            UsersDomainErrors::UserNotFound => "UserNotFound",
        };
        write!(f, "{}", variant_str)
    }
}

impl fmt::Display for UsersErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant_str = match self {
            UsersErrors::DomainError(domain_error) => return write!(f, "{}", domain_error),
            UsersErrors::CommonError(common_error) => return write!(f, "{}", common_error),
            UsersErrors::DatabaseError(_) => "DatabaseError",
        };
        write!(f, "{}", variant_str)
    }
}

impl From<UsersDomainErrors> for UsersErrors {
    fn from(error: UsersDomainErrors) -> Self {
        UsersErrors::DomainError(error)
    }
}

impl From<CommonErrors> for UsersErrors {
    fn from(error: CommonErrors) -> Self {
        UsersErrors::CommonError(error)
    }
}

impl From<rusqlite::Error> for UsersErrors {
    fn from(_: rusqlite::Error) -> Self {
        UsersErrors::DatabaseError(())
    }
}
