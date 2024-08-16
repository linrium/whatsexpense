use crate::api::invoice::invoice_entity::{Discount, InvoiceEntity, Tax};
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::json;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    #[schema(example = "669e5f02b781150b9a578205")]
    pub id: String,
    #[schema(example = "669e5f02b781150b9a578204")]
    pub user_id: String,
    #[schema(example = "669e5f02b781150b9a578203")]
    pub message_id: String,
    #[schema(example = json!([{"amount": 10.0, "rate": 8.0}]))]
    pub taxes: Vec<Tax>,
    #[schema(example = json!([{"name": "Test discount", "amount": 10.0, "rate": 8.0}]))]
    pub discounts: Vec<Discount>,
    #[schema(example = 10.0)]
    pub subtotal: Option<f64>,
    #[schema(example = 10.0)]
    pub total: f64,
    #[schema(example = "USD")]
    pub currency: String,
    #[schema(example = 8432)]
    pub card_number: Option<i16>,
    #[schema(
        example = "/invoices/ae7441fd-1515-4f78-85c9-cbafa7149301/ae7441fd-1515-4f78-85c9-cbafa7149301.jpg"
    )]
    pub media_path: Option<String>,
    #[schema(example = "image/jpeg")]
    pub media_type: Option<String>,
    #[schema(example = "2024-07-22T13:30:42.246014Z")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[schema(example = "2024-07-22T13:30:42.246014Z")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<InvoiceEntity> for Invoice {
    fn from(value: InvoiceEntity) -> Self {
        Self {
            id: value.id.to_hex(),
            user_id: value.user_id.to_hex(),
            message_id: value.message_id.to_hex(),
            taxes: value.taxes,
            discounts: value.discounts,
            subtotal: value.subtotal,
            total: value.total,
            currency: value.currency,
            media_path: value.media_path,
            media_type: value.media_type,
            card_number: value.card_number,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
