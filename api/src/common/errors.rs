use crate::api::auth::AuthError;
use crate::api::category::CategoryError;
use crate::api::exchange_rate::ExchangeRateError;
use crate::api::invoice::InvoiceError;
use crate::api::message::MessageError;
use crate::api::report::ReportError;
use crate::api::transaction::TransactionError;
use crate::api::user::UserError;
use crate::common::mongo::CursorError;
use crate::services::gcp::auth::GCPAuthError;
use crate::services::gcp::vision::GCPVisionError;
use crate::services::jwt::JwtError;
use crate::services::llm::LLMError;
use crate::services::r2::R2Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl ErrorResponse {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    UserError(#[from] UserError),
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    JwtError(#[from] JwtError),
    #[error(transparent)]
    ExchangeRateError(#[from] ExchangeRateError),
    #[error(transparent)]
    TransactionError(#[from] TransactionError),
    #[error(transparent)]
    MessageError(#[from] MessageError),
    #[error(transparent)]
    InvoiceError(#[from] InvoiceError),
    #[error(transparent)]
    CategoryError(#[from] CategoryError),
    #[error(transparent)]
    LLMError(#[from] LLMError),
    #[error(transparent)]
    CursorError(#[from] CursorError),
    #[error(transparent)]
    GCPAuthError(#[from] GCPAuthError),
    #[error(transparent)]
    GCPVisionError(#[from] GCPVisionError),
    #[error(transparent)]
    R2Error(#[from] R2Error),
    #[error(transparent)]
    ReportError(#[from] ReportError),
    #[error("forbidden")]
    Forbidden,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::UserError(e) => e.into_response(),
            Self::AuthError(e) => e.into_response(),
            Self::JwtError(e) => e.into_response(),
            Self::ExchangeRateError(e) => e.into_response(),
            Self::TransactionError(e) => e.into_response(),
            Self::MessageError(e) => e.into_response(),
            Self::InvoiceError(e) => e.into_response(),
            Self::CategoryError(e) => e.into_response(),
            Self::LLMError(e) => e.into_response(),
            Self::CursorError(e) => e.into_response(),
            Self::GCPAuthError(e) => e.into_response(),
            Self::GCPVisionError(e) => e.into_response(),
            Self::R2Error(e) => e.into_response(),
            Self::ReportError(e) => e.into_response(),
            Self::Forbidden => (
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    message: "forbidden".to_string(),
                }),
            )
                .into_response(),
            Self::Unknown(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: e.to_string(),
                }),
            )
                .into_response(),
        }
    }
}
