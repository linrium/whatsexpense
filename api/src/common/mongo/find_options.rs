use std::str::FromStr;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use bson::oid::ObjectId;
use thiserror::Error;

use crate::common::errors::ErrorResponse;

pub struct FindOptions {
    pub limit: Option<i64>,
    pub skip: Option<u64>,
}

impl FindOptions {
    pub fn with_limit(limit: i64) -> Self {
        Self {
            limit: Some(limit),
            skip: None,
        }
    }
}

impl Default for FindOptions {
    fn default() -> Self {
        Self {
            limit: Some(20),
            skip: None,
        }
    }
}

#[derive(Error, Debug)]
pub enum CursorError {
    #[error("invalid id")]
    InvalidId,
}

impl IntoResponse for CursorError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CursorError::InvalidId => (StatusCode::BAD_REQUEST, "invalid id".to_string()),
        };

        let error_message = ErrorResponse {
            message: error_message,
        };

        (status, Json(error_message)).into_response()
    }
}

pub struct Cursor {
    pub id: ObjectId,
}

impl TryFrom<String> for Cursor {
    type Error = CursorError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id = ObjectId::from_str(&value).map_err(|_| CursorError::InvalidId)?;

        Ok(Self { id })
    }
}

impl From<Cursor> for ObjectId {
    fn from(value: Cursor) -> Self {
        value.id
    }
}

impl From<ObjectId> for Cursor {
    fn from(value: ObjectId) -> Self {
        Self { id: value }
    }
}
