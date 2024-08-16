use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseByRangeEntity {
    pub category_id: String,
    pub issued_at: String,
    pub amount: f64,
}
