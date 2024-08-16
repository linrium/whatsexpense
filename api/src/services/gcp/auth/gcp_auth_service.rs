use crate::services::gcp::auth::GCPAuthError;
use crate::settings::GCPConfig;
use anyhow::anyhow;
use async_trait::async_trait;
use gcp_auth::{CustomServiceAccount, TokenProvider};
use redis::AsyncCommands;
use std::sync::Arc;
use tracing::info;

#[async_trait]
pub trait GCPAuthServiceExt: Send + Sync {
    async fn get_access_token(&self) -> Result<String, GCPAuthError>;
}

pub type GCPAuthServiceDyn = Arc<dyn GCPAuthServiceExt + Send + Sync>;

pub struct GCPAuthService {
    service_account: CustomServiceAccount,
    redis_client: redis::Client,
}

impl GCPAuthService {
    pub fn new(gcp_config: GCPConfig, redis_client: redis::Client) -> Self {
        let service_account = CustomServiceAccount::from_json(&gcp_config.service_account)
            .expect("failed to parse service account json");

        Self {
            service_account,
            redis_client,
        }
    }
}

#[async_trait]
impl GCPAuthServiceExt for GCPAuthService {
    async fn get_access_token(&self) -> Result<String, GCPAuthError> {
        let mut con = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| GCPAuthError::Unknown(anyhow!(e)))?;

        let key = "gcp:access_token";
        let token: Option<String> = con
            .get(key)
            .await
            .map_err(|e| GCPAuthError::Unknown(anyhow!(e)))?;

        if let Some(token) = token {
            info!("found token {} in redis", key);
            return Ok(token);
        }

        let scopes = &["https://www.googleapis.com/auth/cloud-vision"];
        let token = self
            .service_account
            .token(scopes)
            .await
            .map_err(|e| GCPAuthError::Unknown(e.into()))?;

        let max_age = 60 * 50;
        let _: () = con
            .set_ex(&key, token.as_str(), max_age)
            .await
            .map_err(|e| GCPAuthError::Unknown(anyhow!(e)))?;
        info!("saved token {} in redis", key);

        Ok(token.as_str().to_string())
    }
}
