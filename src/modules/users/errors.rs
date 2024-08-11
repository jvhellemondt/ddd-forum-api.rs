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
