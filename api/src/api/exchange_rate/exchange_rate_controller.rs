use axum::extract::State;
use axum::http::StatusCode;

use crate::api::state::AppState;
use crate::common::errors::AppError;

pub async fn get_latest(State(state): State<AppState>) -> Result<StatusCode, AppError> {
    state.exchange_rate_service.update_from_source().await?;

    Ok(StatusCode::NO_CONTENT)
}
