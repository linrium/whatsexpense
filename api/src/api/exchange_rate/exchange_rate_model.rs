use crate::api::exchange_rate::ExchangeRateEntity;
use bson::oid::ObjectId;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRate {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub code: String,
    pub value: f64,
    pub last_updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<ExchangeRateEntity> for ExchangeRate {
    fn from(entity: ExchangeRateEntity) -> Self {
        Self {
            id: entity.id,
            code: entity.code,
            value: entity.value,
            last_updated_at: entity.last_updated_at,
        }
    }
}
