use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use s3::creds::Credentials;
use s3::{Bucket, Region};

use crate::services::r2::constants::R2Error;
use crate::settings::R2Config;

#[async_trait]
pub trait R2ServiceExt: Send + Sync {
    async fn upload_object(
        &self,
        bucket: String,
        path: String,
        content: &[u8],
        content_type: &str,
    ) -> Result<String, R2Error>;
    async fn presign_get(&self, path: String) -> Result<String, R2Error>;
    async fn presign_post(&self, path: String) -> Result<String, R2Error>;
}

pub type R2ServiceDyn = Arc<dyn R2ServiceExt + Send + Sync>;

pub struct R2Service {
    pub bucket: Bucket,
}

impl R2Service {
    pub fn new(r2_config: R2Config) -> Self {
        let region = Region::R2 {
            account_id: r2_config.account_id,
        };
        let credentials = Credentials::new(
            Some(r2_config.access_key.as_str()),
            Some(r2_config.secret_key.as_str()),
            None,
            None,
            None,
        )
        .expect("failed to create credentials");
        let bucket = Bucket::new("invoices", region, credentials).expect("failed to create bucket");

        Self { bucket }
    }
}

impl R2Service {
    const DEFAULT_EXPIRED_SEC: u32 = 3600;
}

#[async_trait]
impl R2ServiceExt for R2Service {
    async fn upload_object(
        &self,
        bucket: String,
        path: String,
        content: &[u8],
        content_type: &str,
    ) -> Result<String, R2Error> {
        self.bucket
            .put_object_with_content_type(path.clone(), content, content_type)
            .await
            .map_err(|e| R2Error::Unknown(anyhow!(e)))?;

        let path = format!("{}/{}", bucket, path);
        Ok(path)
    }

    async fn presign_get(&self, path: String) -> Result<String, R2Error> {
        self.bucket
            .presign_get(path, Self::DEFAULT_EXPIRED_SEC, None)
            .await
            .map_err(|e| R2Error::Unknown(anyhow!(e)))
    }

    async fn presign_post(&self, path: String) -> Result<String, R2Error> {
        self.bucket
            .presign_put(path, Self::DEFAULT_EXPIRED_SEC, None)
            .await
            .map_err(|e| R2Error::Unknown(anyhow!(e)))
    }
}
