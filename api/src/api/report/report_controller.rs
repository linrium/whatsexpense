use std::str::FromStr;

use anyhow::anyhow;
use axum::extract::{Query, State};
use axum::{Extension, Json};
use chrono::{NaiveDate, TimeZone, Utc};
use utoipa::OpenApi;

use crate::api::report::{ExpenseByRange, ReportError, ReportExpensesByRangeQuery};
use crate::api::state::AppState;
use crate::api::user::User;
use crate::common::errors::AppError;
use crate::object_id;

#[utoipa::path(
    get,
    path = "/range",
    params(
        ReportExpensesByRangeQuery,
    ),
    responses(
        (status = 200, description = "List messages successfully", body = [ExpenseByRange]),
    )
)]
pub async fn report_expenses_by_range(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Query(query): Query<ReportExpensesByRangeQuery>,
) -> Result<Json<Vec<ExpenseByRange>>, AppError> {
    let from = NaiveDate::from_str(&query.from).map_err(|e| AppError::Unknown(e.into()))?;
    let to = NaiveDate::from_str(&query.to).map_err(|e| AppError::Unknown(e.into()))?;
    let from_datetime = Utc.from_utc_datetime(
        &from
            .and_hms_opt(0, 0, 0)
            .ok_or(AppError::Unknown(anyhow!("invalid date")))?,
    );
    let to_datetime = Utc.from_utc_datetime(
        &to.and_hms_opt(0, 0, 0)
            .ok_or(AppError::Unknown(anyhow!("invalid date")))?,
    );
    let tomorrow = Utc::now()
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .ok_or(AppError::Unknown(anyhow!("invalid date")))?
        + chrono::Duration::days(1);
    let tomorrow = Utc.from_utc_datetime(&tomorrow);

    if from_datetime > to_datetime {
        return Err(ReportError::InvalidDateRange.into());
    }

    if from_datetime == to_datetime {
        return Err(ReportError::InvalidDateRange.into());
    }

    if to_datetime >= tomorrow {
        return Err(ReportError::InvalidDateRange.into());
    }

    if to_datetime - from_datetime > chrono::Duration::days(30) {
        return Err(ReportError::InvalidDateRange.into());
    }

    let expenses = state
        .report_service
        .get_expenses_by_range(object_id!(&user.id), from_datetime, to_datetime)
        .await?;

    Ok(Json(expenses))
}

#[derive(OpenApi)]
#[openapi(
    paths(report_expenses_by_range),
    components(
        schemas(
            ExpenseByRange,
        )
    ),
    tags(
        (name = "crate::api::message", description = "Message API")
    )
)]
pub struct ReportApiDoc;
