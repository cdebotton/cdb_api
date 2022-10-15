use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(thiserror::Error, Debug, utoipa::ToSchema)]
pub enum Error {
    #[error("Not found")]
    NotFound,
    #[error("Unable to start server")]
    InternalError,
    #[error("Invalid JWT")]
    InvalidToken,
    #[error("Validation error")]
    ValidationError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        use Error::*;

        let status_code = match self {
            InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            NotFound => StatusCode::NOT_FOUND,
            InvalidToken | ValidationError => StatusCode::BAD_REQUEST,
        };

        let body = Json(json!({ "error": self.to_string() }));

        (status_code, body).into_response()
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(_validation: validator::ValidationErrors) -> Self {
        Error::ValidationError
    }
}

impl From<sqlx::Error> for Error {
    fn from(_err: sqlx::Error) -> Self {
        Error::InternalError
    }
}

impl From<hyper::Error> for Error {
    fn from(_err: hyper::Error) -> Self {
        Error::InternalError
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(_err: jsonwebtoken::errors::Error) -> Self {
        Error::InternalError
    }
}
