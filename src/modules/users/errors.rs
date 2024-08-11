use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum UsersErrors {
    UsernameAlreadyTaken,
    EmailAlreadyInUse,
    UserNotFound,
}
