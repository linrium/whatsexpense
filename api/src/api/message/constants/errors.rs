use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use thiserror::Error;

use crate::common::errors::ErrorResponse;

#[derive(Error, Debug)]
pub enum MessageError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
    #[error("message not found")]
    NotFound,
}

impl IntoResponse for MessageError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::Unknown(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::NotFound => (StatusCode::NOT_FOUND, "message not found".to_string()),
        };

        let error_response = ErrorResponse { message };

        (status, Json(error_response)).into_response()
    }
}
