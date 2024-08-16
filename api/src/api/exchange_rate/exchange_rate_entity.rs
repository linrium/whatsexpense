use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRateEntity {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub code: String,
    pub value: f64,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub last_updated_at: chrono::DateTime<chrono::Utc>,
}
