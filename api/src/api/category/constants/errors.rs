use crate::common::errors::ErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CategoryError {
    #[error("category not found")]
    NotFound,
}

impl IntoResponse for CategoryError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
        };

        let error_response = ErrorResponse { message };

        (status, Json(error_response)).into_response()
    }
}
