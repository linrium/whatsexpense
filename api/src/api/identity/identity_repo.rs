use std::sync::Arc;

use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;

use crate::api::identity::*;

#[async_trait]
pub trait IdentityRepoExt: Send + Sync {
    async fn find_by_id(&self, id: ObjectId) -> Result<Option<IdentityEntity>, IdentityError>;
    async fn insert_one(&self, data: InsertIdentityInput) -> Result<IdentityEntity, IdentityError>;
}

pub type IdentityRepoDyn = Arc<dyn IdentityRepoExt + Send + Sync>;

#[derive(Clone)]
pub struct IdentityRepo {
    pub collection: Collection<IdentityEntity>,
}

#[async_trait]
impl IdentityRepoExt for IdentityRepo {
    async fn find_by_id(&self, id: ObjectId) -> Result<Option<IdentityEntity>, IdentityError> {
        self.collection
            .find_one(doc! { "_id": id })
            .await
            .map_err(|_| IdentityError::InternalError)
    }

    async fn insert_one(&self, data: InsertIdentityInput) -> Result<IdentityEntity, IdentityError> {
        let doc = IdentityEntity {
            id: ObjectId::new(),
            sub: data.sub,
            email: data.email,
            user_id: data.user_id,
            provider: data.provider,
            identity_data: data.identity_data,
            last_sign_in_at: chrono::Utc::now(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.collection
            .insert_one(&doc)
            .await
            .map_err(|_| IdentityError::InternalError)?;

        Ok(doc)
    }
}
