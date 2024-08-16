use bson::oid::ObjectId;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub struct UpdateTransactionData {
    pub id: ObjectId,
    pub user_id: ObjectId,
    pub title: Option<String>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub category_id: Option<String>,
    pub type_: Option<String>,
    pub unit: Option<String>,
    pub quantity: Option<f32>,
    pub issued_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct UpdateTransactionInput {
    pub id: ObjectId,
    pub user_id: ObjectId,
    pub title: Option<String>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub category_id: Option<String>,
    pub type_: Option<String>,
    pub unit: Option<String>,
    pub quantity: Option<f32>,
    pub issued_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTransactionBody {
    #[schema(example = "5f8d9f0a0b0c0d0e0f00")]
    pub id: String,
    #[schema(example = "What I bought")]
    pub title: Option<String>,
    #[schema(example = 100)]
    pub amount: Option<i64>,
    #[schema(example = "USD")]
    pub currency: Option<String>,
    #[schema(example = "groceries")]
    pub category_id: Option<String>,
    #[schema(example = "income")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[schema(example = "kg")]
    pub unit: Option<String>,
    #[schema(example = 1.0)]
    pub quantity: Option<f32>,
    #[schema(example = "2024-07-22T13:30:42.246017Z")]
    pub issued_at: Option<chrono::DateTime<chrono::Utc>>,
}
