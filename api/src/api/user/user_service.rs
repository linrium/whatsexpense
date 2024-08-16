use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use bson::oid::ObjectId;
use redis::AsyncCommands;
use tracing::{info, trace};

use crate::api::asset::{validate_currency_code, validate_language_code, validate_region_code};
use crate::api::user::*;
use crate::common::errors::AppError;

#[async_trait]
pub trait UserServiceExt: Send + Sync {
    async fn find_by_email(&self, email: String) -> Result<Option<User>, AppError>;
    async fn find_by_id(&self, id: ObjectId) -> Result<Option<User>, AppError>;
    async fn insert_one(&self, data: CreateUserInput) -> Result<User, AppError>;
    async fn update_by_id(
        &self,
        id: ObjectId,
        data: UpdateUserInput,
    ) -> Result<Option<User>, AppError>;
    async fn soft_delete_by_id(&self, id: ObjectId) -> Result<(), AppError>;
}

pub type UserServiceDyn = Arc<dyn UserServiceExt + Send + Sync>;

#[derive(Clone)]
pub struct UserService {
    pub repo: UserRepoDyn,
    pub redis_client: redis::Client,
}

#[async_trait]
impl UserServiceExt for UserService {
    async fn find_by_email(&self, email: String) -> Result<Option<User>, AppError> {
        self.repo
            .find_by_email(email)
            .await
            .map_err(Into::into)
            .map(|v| v.map(Into::into))
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<Option<User>, AppError> {
        let mut con = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| UserError::Unknown(anyhow!(e)))?;
        let key = format!("user:{}", id);
        let user: Option<User> = con
            .get(&key)
            .await
            .map_err(|e| UserError::Unknown(anyhow!(e)))?;
        if user.is_some() {
            trace!("found user {} in redis", id);
            return Ok(user);
        }

        let user: Option<User> = self.repo.find_by_id(id).await.map(|v| v.map(User::from))?;

        let max_age = 24 * 60 * 60;
        let _: () = con
            .set_ex(&key, user.clone(), max_age)
            .await
            .map_err(|e| UserError::Unknown(anyhow!(e)))?;
        trace!("saved user {} in redis", id);

        Ok(user)
    }

    async fn insert_one(&self, data: CreateUserInput) -> Result<User, AppError> {
        if validate_currency_code(&data.currency).is_none() {
            return Err(UserError::InvalidCurrencyCode.into());
        }

        if data.regions.len() > 1 {
            return Err(UserError::RegionExceedsLimit.into());
        }

        if validate_language_code(&data.language).is_none() {
            return Err(UserError::InvalidRegionCode.into());
        }

        for region in &data.regions {
            if validate_region_code(region).is_none() {
                return Err(UserError::InvalidRegionCode.into());
            }
        }

        self.repo
            .insert_one(data.into())
            .await
            .map(Into::into)
            .map_err(Into::into)
    }

    async fn update_by_id(
        &self,
        id: ObjectId,
        data: UpdateUserInput,
    ) -> Result<Option<User>, AppError> {
        if let Some(currency) = &data.currency {
            if validate_currency_code(currency).is_none() {
                return Err(UserError::InvalidCurrencyCode.into());
            }
        }

        if let Some(language) = &data.language {
            if validate_language_code(language).is_none() {
                return Err(UserError::InvalidRegionCode.into());
            }
        }

        if let Some(regions) = &data.regions {
            for region in regions {
                if validate_region_code(region).is_none() {
                    return Err(UserError::InvalidRegionCode.into());
                }
            }
        }

        let user = self
            .repo
            .update_by_id(id, data.into())
            .await
            .map(|v| v.map(User::from))?;
        if let Some(user) = &user {
            let mut con = self
                .redis_client
                .get_multiplexed_async_connection()
                .await
                .map_err(|e| UserError::Unknown(anyhow!(e)))?;
            let key = format!("user:{}", id);
            let max_age = 24 * 60 * 60;
            let _: () = con
                .set_ex(&key, user.clone(), max_age)
                .await
                .map_err(|e| UserError::Unknown(anyhow!(e)))?;
            info!("saved user {} in redis", id);
        }

        Ok(user)
    }

    async fn soft_delete_by_id(&self, id: ObjectId) -> Result<(), AppError> {
        self.repo.soft_delete_by_id(id).await?;

        let mut con = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| UserError::Unknown(anyhow!(e)))?;
        let key = format!("user:{}", id);
        let _: () = con
            .del(&key)
            .await
            .map_err(|e| UserError::Unknown(anyhow!(e)))?;
        info!("deleted user {} from redis", id);

        Ok(())
    }
}
