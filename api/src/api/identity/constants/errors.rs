use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::common::errors::ErrorResponse;

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("internal error")]
    InternalError,
}

impl IntoResponse for IdentityError {
    fn into_response(self) -> Response {
        let (code, message) = (StatusCode::INTERNAL_SERVER_ERROR, self.to_string());

        let error_response = ErrorResponse { message };

        (code, axum::Json(error_response)).into_response()
    }
}
