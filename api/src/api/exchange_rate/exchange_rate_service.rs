use std::sync::Arc;

use async_trait::async_trait;

use crate::api::exchange_rate::*;
use crate::common::errors::AppError;
use crate::services::currencyapi::{CurrencyApiServiceDyn, LatestExchangeRate};

#[async_trait]
pub trait ExchangeRateServiceExt: Send + Sync {
    async fn latest(
        &self,
        base_currency: &str,
    ) -> Result<(chrono::DateTime<chrono::Utc>, Vec<LatestExchangeRate>), AppError>;
    async fn insert_many(
        &self,
        items: Vec<CreateExchangeRateInput>,
    ) -> Result<Vec<ExchangeRate>, AppError>;
    async fn update_from_source(&self) -> Result<Vec<ExchangeRate>, AppError>;
}

pub type ExchangeRateServiceDyn = Arc<dyn ExchangeRateServiceExt + Send + Sync>;

pub struct ExchangeRateService {
    pub currencyapi_service: CurrencyApiServiceDyn,
    pub repo: ExchangeRateRepoDyn,
}

impl ExchangeRateService {
    const BASE_CURRENCY: &'static str = "USD";
}

#[async_trait]
impl ExchangeRateServiceExt for ExchangeRateService {
    async fn latest(
        &self,
        base_currency: &str,
    ) -> Result<(chrono::DateTime<chrono::Utc>, Vec<LatestExchangeRate>), AppError> {
        let result = self
            .currencyapi_service
            .latest(base_currency)
            .await
            .map_err(|_| ExchangeRateError::Unknown)?;

        Ok(result)
    }

    async fn insert_many(
        &self,
        items: Vec<CreateExchangeRateInput>,
    ) -> Result<Vec<ExchangeRate>, AppError> {
        self.repo
            .insert_many(
                items
                    .into_iter()
                    .map(|item| CreateExchangeRateData {
                        last_updated_at: item.last_updated_at,
                        code: item.code,
                        value: item.value,
                    })
                    .collect(),
            )
            .await
            .map(|items| items.into_iter().map(ExchangeRate::from).collect())
            .map_err(Into::into)
    }

    async fn update_from_source(&self) -> Result<Vec<ExchangeRate>, AppError> {
        let (last_updated_at, exchange_rates) = self.latest(Self::BASE_CURRENCY).await?;

        let items = exchange_rates
            .into_iter()
            .map(|item| CreateExchangeRateInput {
                last_updated_at,
                code: item.code,
                value: item.value,
            })
            .collect::<Vec<_>>();

        let exchange_rates = self.insert_many(items).await?;

        Ok(exchange_rates)
    }
}
