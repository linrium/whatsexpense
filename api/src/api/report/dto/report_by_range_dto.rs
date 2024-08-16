use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct ReportExpensesByRangeQuery {
    pub from: String,
    pub to: String,
}
