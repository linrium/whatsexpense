use crate::services::currencyapi::*;
use crate::settings::Settings;
use async_trait::async_trait;
use reqwest::Url;
use std::sync::Arc;

#[async_trait]
pub trait CurrencyApiServiceExt: Send + Sync {
    async fn latest(
        &self,
        code: &str,
    ) -> Result<(chrono::DateTime<chrono::Utc>, Vec<LatestExchangeRate>), CurrencyApiError>;
}

pub type CurrencyApiServiceDyn = Arc<dyn CurrencyApiServiceExt + Send + Sync>;

pub struct CurrencyApiService {
    pub settings: Arc<Settings>,
    pub http_client: reqwest::Client,
}

#[async_trait]
impl CurrencyApiServiceExt for CurrencyApiService {
    async fn latest(
        &self,
        base_currency: &str,
    ) -> Result<(chrono::DateTime<chrono::Utc>, Vec<LatestExchangeRate>), CurrencyApiError> {
        let url = self.settings.currencyapi.url.clone();
        let url = Url::parse_with_params(
            &url,
            vec![("base_currency", base_currency), ("type", "fiat")],
        )
        .map_err(|e| CurrencyApiError::Unknown(e.into()))?;

        let resp = self
            .http_client
            .get(url)
            .header("apiKey", self.settings.currencyapi.api_key.clone())
            .send()
            .await
            .map_err(|e| CurrencyApiError::Unknown(e.into()))?
            .json::<LatestResult>()
            .await
            .map_err(|e| CurrencyApiError::Unknown(e.into()))?;

        let values = resp.data.values().cloned().collect::<Vec<_>>();

        Ok((resp.meta.last_updated_at, values))
    }
}
