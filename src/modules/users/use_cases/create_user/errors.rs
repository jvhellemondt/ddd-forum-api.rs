use crate::modules::users::errors::UsersErrors;
use crate::shared::common::errors::CommonErrors;

#[derive(Debug)]
pub enum CreateUserErrors {
    UsersError(UsersErrors),
    CommonError(CommonErrors),
    DatabaseError(()),
}

impl From<UsersErrors> for CreateUserErrors {
    fn from(error: UsersErrors) -> Self {
        CreateUserErrors::UsersError(error)
    }
}

impl From<CommonErrors> for CreateUserErrors {
    fn from(error: CommonErrors) -> Self {
        CreateUserErrors::CommonError(error)
    }
}

impl From<rusqlite::Error> for CreateUserErrors {
    fn from(_: rusqlite::Error) -> Self {
        CreateUserErrors::DatabaseError(())
    }
}
