use crate::common::errors::ErrorResponse;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvoiceError {
    #[error("invoice not found")]
    NotFound,
    #[error("unsupported content type")]
    UnsupportedContentType,
    #[error("no attachment")]
    NoAttachment,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for InvoiceError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            Self::UnsupportedContentType => (StatusCode::UNSUPPORTED_MEDIA_TYPE, self.to_string()),
            Self::NoAttachment => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::Unknown(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let error_response = ErrorResponse { message };

        (status, Json(error_response)).into_response()
    }
}
