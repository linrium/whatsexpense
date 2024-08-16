use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEntity {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub message_id: ObjectId,
    pub user_id: ObjectId,
    pub invoice_id: ObjectId,
    pub title: String,
    pub amount: f64,
    pub currency: String,
    pub category_id: String,
    pub r#type: String,
    pub unit: Option<String>,
    pub quantity: f64,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub issued_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
