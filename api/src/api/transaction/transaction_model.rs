use crate::api::transaction::TransactionEntity;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[schema(example = "669fb456ce6a5cbb87195a60")]
    pub id: String,
    #[schema(example = "669fb456ce6a5cbb87195a5e")]
    pub message_id: String,
    #[schema(example = "66990b1947d76ec3781adc9d")]
    pub user_id: String,
    #[schema(example = "Dinner")]
    pub title: String,
    #[schema(example = 100.00)]
    pub amount: f64,
    #[schema(example = "USD")]
    pub currency: String,
    #[schema(example = "dinning_out")]
    pub category_id: String,
    #[schema(example = "outcome")]
    pub r#type: String,
    #[schema(example = "kg")]
    pub unit: Option<String>,
    #[schema(example = 1.0)]
    pub quantity: f64,
    #[schema(example = "2024-07-22T13:30:42.246017Z")]
    pub issued_at: chrono::DateTime<chrono::Utc>,
    #[schema(example = "2024-07-22T13:30:42.246017Z")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[schema(example = "2024-07-22T13:30:42.246017Z")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<TransactionEntity> for Transaction {
    fn from(value: TransactionEntity) -> Self {
        Self {
            id: value.id.to_hex(),
            message_id: value.message_id.to_hex(),
            user_id: value.user_id.to_hex(),
            title: value.title,
            amount: value.amount,
            currency: value.currency,
            category_id: value.category_id,
            r#type: value.r#type,
            unit: value.unit,
            quantity: value.quantity,
            issued_at: value.issued_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
