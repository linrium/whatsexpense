use std::sync::Arc;

use async_trait::async_trait;
use bson::oid::ObjectId;
use chrono::{DurationRound, TimeDelta};
use mongodb::Collection;

use crate::api::exchange_rate::{CreateExchangeRateData, ExchangeRateEntity, ExchangeRateError};

#[async_trait]
pub trait ExchangeRateRepoExt: Send + Sync {
    async fn insert_many(
        &self,
        items: Vec<CreateExchangeRateData>,
    ) -> Result<Vec<ExchangeRateEntity>, ExchangeRateError>;
}

pub type ExchangeRateRepoDyn = Arc<dyn ExchangeRateRepoExt + Send + Sync>;

#[derive(Clone)]
pub struct ExchangeRateRepo {
    pub collection: Collection<ExchangeRateEntity>,
}

#[async_trait]
impl ExchangeRateRepoExt for ExchangeRateRepo {
    async fn insert_many(
        &self,
        items: Vec<CreateExchangeRateData>,
    ) -> Result<Vec<ExchangeRateEntity>, ExchangeRateError> {
        let docs = items
            .into_iter()
            .map(|item| ExchangeRateEntity {
                id: ObjectId::new(),
                code: item.code,
                value: item.value,
                last_updated_at: item
                    .last_updated_at
                    .duration_trunc(TimeDelta::try_days(1).unwrap())
                    .unwrap(),
            })
            .collect::<Vec<_>>();

        self.collection
            .insert_many(docs.clone())
            .ordered(false)
            .await
            .map_err(|_| ExchangeRateError::CreationFailed)?;

        Ok(docs)
    }
}
