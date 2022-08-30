use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("There was a database error")]
    DBError(#[from] sqlx::Error),
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Unable to create token")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
}

impl AuthError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AuthError::DBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::WrongCredentials => StatusCode::UNAUTHORIZED,
            AuthError::MissingCredentials | AuthError::InvalidToken => StatusCode::BAD_REQUEST,
            AuthError::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let error_message = self.to_string();
        let body = Json(json!({ "error": error_message }));

        (self.status_code(), body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use axum::{http::StatusCode, response::IntoResponse};

    use super::AuthError;

    #[test]
    fn it_returns_status_code_with_response() {
        let error = AuthError::WrongCredentials;
        let resp = error.into_response();

        let status_code = resp.status();
        assert_eq!(status_code, StatusCode::UNAUTHORIZED);
    }
}
