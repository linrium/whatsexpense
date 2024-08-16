use crate::api::auth::types::Provider;
use crate::api::identity::IdentityEntity;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub id: String,
    pub sub: String,
    pub identity_data: serde_json::Value,
    pub provider: Provider,
    pub email: Option<String>,
    pub user_id: ObjectId,
    pub last_sign_in_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<IdentityEntity> for Identity {
    fn from(entity: IdentityEntity) -> Self {
        Self {
            id: entity.id.to_string(),
            sub: entity.sub,
            identity_data: entity.identity_data,
            provider: entity.provider,
            email: entity.email,
            user_id: entity.user_id,
            last_sign_in_at: entity.last_sign_in_at,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
