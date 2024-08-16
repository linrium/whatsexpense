use crate::api::auth::types::Provider;
use bson::oid::ObjectId;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InsertIdentityInput {
    pub name: String,
    pub email: Option<String>,
    pub user_id: ObjectId,
    pub provider: Provider,
    pub sub: String,
    pub identity_data: serde_json::Value,
}
