use std::sync::Arc;

use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

use crate::api::identity::*;

#[async_trait]
pub trait IdentityServiceExt: Send + Sync {
    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Identity>, IdentityError>;
    async fn insert_one(&self, data: InsertIdentityInput) -> Result<Identity, IdentityError>;
}

pub type IdentityServiceDyn = Arc<dyn IdentityServiceExt + Send + Sync>;

#[derive(Clone)]
pub struct IdentityService {
    pub repo: IdentityRepoDyn,
}

#[async_trait]
impl IdentityServiceExt for IdentityService {
    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Identity>, IdentityError> {
        self.repo
            .find_by_id(id)
            .await
            .map(|opt| opt.map(Into::into))
    }

    async fn insert_one(&self, data: InsertIdentityInput) -> Result<Identity, IdentityError> {
        self.repo.insert_one(data).await.map(Into::into)
    }
}
