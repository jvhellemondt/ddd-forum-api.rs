use thiserror::Error;
use serde::Serialize;
use crate::shared::common::errors::CommonErrors;

#[derive(Serialize, Debug, Error)]
pub enum UsersDomainErrors {
    #[error("Username is already taken")]
    UsernameAlreadyTaken,

    #[error("Email is already in use")]
    EmailAlreadyInUse,

    #[error("User not found")]
    UserNotFound,
}

#[derive(Debug, Error)]
pub enum UsersModuleErrors {
    #[error("Domain error: {0}")]
    DomainError(#[from] UsersDomainErrors),

    #[error("Common error: {0}")]
    CommonError(#[from] CommonErrors),

}
