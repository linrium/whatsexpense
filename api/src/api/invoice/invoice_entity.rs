use crate::api::infer::models::{DiscountTool, TaxTool};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Tax {
    #[schema(example = 10.0)]
    pub amount: f32,
    #[schema(example = 8.0)]
    pub rate: f32,
}

impl From<TaxTool> for Tax {
    fn from(tax: TaxTool) -> Self {
        Self {
            amount: tax.amount,
            rate: tax.rate,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Discount {
    #[schema(example = "Test discount")]
    pub name: String,
    #[schema(example = 10.0)]
    pub amount: f32,
    #[schema(example = 8.0)]
    pub rate: f32,
}

impl From<DiscountTool> for Discount {
    fn from(discount: DiscountTool) -> Self {
        Self {
            name: discount.name,
            amount: discount.amount,
            rate: discount.rate,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceEntity {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user_id: ObjectId,
    pub message_id: ObjectId,
    pub taxes: Vec<Tax>,
    pub discounts: Vec<Discount>,
    pub subtotal: Option<f64>,
    pub total: f64,
    pub currency: String,
    pub card_number: Option<i16>,
    pub media_path: Option<String>,
    pub media_type: Option<String>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
