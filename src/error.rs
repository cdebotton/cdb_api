use crate::jwt::AuthError;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("There was an error with authorization")]
    AuthError(#[from] AuthError),
}
