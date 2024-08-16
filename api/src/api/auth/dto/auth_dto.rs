use serde::Serialize;
#[allow(unused_imports)]
use serde_json::json;
use utoipa::ToSchema;

use crate::api::user::User;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthPayload {
    #[schema(example = "eyJhb...")]
    pub access_token: String,
    pub refresh_token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: User,
}

impl From<AuthResponse> for AuthPayload {
    fn from(v: AuthResponse) -> Self {
        Self {
            access_token: v.access_token,
            refresh_token: v.refresh_token,
            user: v.user,
        }
    }
}
