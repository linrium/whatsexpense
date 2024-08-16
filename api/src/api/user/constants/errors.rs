use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use thiserror::Error;

use crate::common::errors::ErrorResponse;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("user not found")]
    NotFound,
    #[error("invalid currency code")]
    InvalidCurrencyCode,
    #[error("invalid region code")]
    InvalidRegionCode,
    #[error("region exceeds limit")]
    RegionExceedsLimit,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            Self::InvalidCurrencyCode => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::InvalidRegionCode => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::RegionExceedsLimit => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::Unknown(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let error_response = ErrorResponse { message };

        (status, Json(error_response)).into_response()
    }
}
