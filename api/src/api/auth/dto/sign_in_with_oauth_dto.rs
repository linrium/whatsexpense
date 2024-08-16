use crate::api::auth::types::Provider;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct SignInWithOauthBody {
    #[schema(example = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjQ...")]
    pub code: String,
    #[schema(example = "Apple")]
    pub provider: Provider,
}
