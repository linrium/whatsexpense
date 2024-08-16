use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserEntity {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub email: String,
    pub full_name: String,
    pub username: String,
    pub encrypted_password: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub language: String,
    pub regions: Vec<String>,
    pub currency: String,
    #[serde_as(as = "Option<bson::DateTime>")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
