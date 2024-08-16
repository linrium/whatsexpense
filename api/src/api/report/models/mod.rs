use crate::api::report::ExpenseByRangeEntity;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExpenseByRange {
    #[schema(example = "groceries")]
    pub category_id: String,
    #[schema(example = "2024-07-20")]
    pub issued_at: String,
    #[schema(example = 100.0)]
    pub amount: f64,
}

impl From<ExpenseByRangeEntity> for ExpenseByRange {
    fn from(entity: ExpenseByRangeEntity) -> Self {
        Self {
            category_id: entity.category_id,
            issued_at: entity.issued_at,
            amount: entity.amount,
        }
    }
}
