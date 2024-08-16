use crate::common::errors::ErrorResponse;
use async_trait::async_trait;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Error)]
pub enum ValidatedError {
    #[error(transparent)]
    ValidError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ValidatedError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<T>::from_request(req, state).await?;
        data.validate()?;
        Ok(Self(data))
    }
}

impl IntoResponse for ValidatedError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ValidatedError::ValidError(_) => {
                let message = format!("input validation error: [{self}]").replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ValidatedError::JsonRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        let error_message = ErrorResponse { message };

        (status, Json(error_message)).into_response()
    }
}
