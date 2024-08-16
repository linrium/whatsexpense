use crate::common::errors::ErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("transaction not found")]
    NotFound,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for TransactionError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            Self::Unknown(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let error_response = ErrorResponse { message };

        (status, Json(error_response)).into_response()
    }
}
