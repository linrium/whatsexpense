use crate::common::errors::ErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CurrencyApiError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for CurrencyApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::Unknown(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let error_message = ErrorResponse { message };

        (status, Json(error_message)).into_response()
    }
}
