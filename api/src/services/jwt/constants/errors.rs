use crate::common::errors::ErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("token not found")]
    NotFound,
    #[error("unable to decode token")]
    UnableToDecode,
    #[error("unable to encode token")]
    UnableToEncode,
    #[error("invalid sub")]
    InvalidSub,
    #[error("token expired")]
    Expired,
}

impl IntoResponse for JwtError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::UnableToDecode => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Self::UnableToEncode => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Self::InvalidSub => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::Expired => (StatusCode::UNAUTHORIZED, self.to_string()),
        };

        let error_message = ErrorResponse { message };

        (status, Json(error_message)).into_response()
    }
}
