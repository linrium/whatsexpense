use crate::common::errors::ErrorResponse;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReportError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
    #[error("invalid date range")]
    InvalidDateRange,
}

impl IntoResponse for ReportError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::Unknown(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::InvalidDateRange => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        let error_response = ErrorResponse { message };

        (status, Json(error_response)).into_response()
    }
}
