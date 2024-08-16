use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct RenewAccessTokenQuery {
    pub refresh_token: String,
}
