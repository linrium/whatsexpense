use crate::common::errors::ErrorResponse;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("invalid token")]
    InvalidToken,
    #[error("invalid credentials")]
    WrongCredentials,
    #[error("token creation error")]
    TokenCreation,
    #[error("token expired")]
    TokenExpired,
    #[error("missing credentials")]
    MissingCredentials,
    #[error("hashing error")]
    Hashing,
    #[error("invalid password")]
    InvalidPassword,
    #[error("refresh token expired")]
    RefreshTokenExpired,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::InvalidToken => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::TokenExpired => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            Self::WrongCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Self::MissingCredentials => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::InvalidPassword => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::Hashing => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Self::RefreshTokenExpired => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::Unknown(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let error_message = ErrorResponse { message };

        (status, Json(error_message)).into_response()
    }
}
