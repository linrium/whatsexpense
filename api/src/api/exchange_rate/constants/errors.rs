use crate::common::errors::ErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExchangeRateError {
    #[error("unknown error")]
    Unknown,
    #[error("failed to create exchange rate")]
    CreationFailed,
}

impl IntoResponse for ExchangeRateError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::Unknown => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Self::CreationFailed => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let error_response = ErrorResponse { message };

        (status, Json(error_response)).into_response()
    }
}
