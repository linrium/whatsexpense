use crate::api::user::UserEntity;
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::json;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, FromRedisValue, ToRedisArgs, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[schema(example = "66990b1947d76ec3781adc9d")]
    pub id: String,
    #[schema(example = "test@test.com")]
    pub email: String,
    #[schema(example = "Test User")]
    pub full_name: String,
    #[schema(example = "test")]
    pub username: String,
    #[serde(skip)]
    pub encrypted_password: String,
    #[schema(example = "Test")]
    pub given_name: String,
    #[schema(example = "User")]
    pub family_name: String,
    #[schema(example = "https://example.com/image.png")]
    pub picture: String,
    #[schema(example = "en")]
    pub language: String,
    #[schema(example = json!(["us", "uk"]))]
    pub regions: Vec<String>,
    #[schema(example = "VND")]
    pub currency: String,
    #[schema(example = "2024-07-18T12:31:21.818Z")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[schema(example = "2024-07-18T12:31:21.818Z")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<UserEntity> for User {
    fn from(value: UserEntity) -> Self {
        Self {
            id: value.id.to_string(),
            email: value.email,
            full_name: value.full_name,
            username: value.username,
            encrypted_password: value.encrypted_password,
            given_name: value.given_name,
            family_name: value.family_name,
            picture: value.picture,
            language: value.language,
            regions: value.regions,
            currency: value.currency,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
