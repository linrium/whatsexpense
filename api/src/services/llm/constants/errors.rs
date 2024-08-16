use crate::common::errors::ErrorResponse;
use async_openai::error::OpenAIError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LLMError {
    #[error(transparent)]
    OpenAIError(#[from] OpenAIError),
    #[error("empty response")]
    EmptyResponse,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for LLMError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::OpenAIError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::EmptyResponse => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::Unknown(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let error_response = ErrorResponse { message };

        (status, Json(error_response)).into_response()
    }
}
