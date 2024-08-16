use std::sync::Arc;

use crate::api::user::*;
use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::ReturnDocument;
use mongodb::Collection;

#[async_trait]
pub trait UserRepoExt: Send + Sync {
    async fn find_by_email(&self, email: String) -> Result<Option<UserEntity>, UserError>;
    async fn find_by_id(&self, id: ObjectId) -> Result<Option<UserEntity>, UserError>;
    async fn insert_one(&self, data: CreateUserData) -> Result<UserEntity, UserError>;
    async fn update_by_id(
        &self,
        id: ObjectId,
        data: UpdateUserData,
    ) -> Result<Option<UserEntity>, UserError>;
    async fn soft_delete_by_id(&self, id: ObjectId) -> Result<(), UserError>;
}

pub type UserRepoDyn = Arc<dyn UserRepoExt + Send + Sync>;

#[derive(Clone)]
pub struct UserRepo {
    pub collection: Collection<UserEntity>,
}

#[async_trait]
impl UserRepoExt for UserRepo {
    async fn find_by_email(&self, email: String) -> Result<Option<UserEntity>, UserError> {
        self.collection
            .find_one(doc! { "email": email, "deletedAt": null })
            .await
            .map_err(|e| UserError::Unknown(e.into()))
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<Option<UserEntity>, UserError> {
        self.collection
            .find_one(doc! { "_id": id, "deletedAt": null })
            .await
            .map_err(|e| UserError::Unknown(e.into()))
    }

    async fn insert_one(&self, data: CreateUserData) -> Result<UserEntity, UserError> {
        let document = UserEntity {
            id: ObjectId::new(),
            full_name: data.full_name,
            username: data.username,
            currency: data.currency,
            language: data.language,
            regions: data.regions,
            email: data.email,
            family_name: data.family_name,
            given_name: data.given_name,
            picture: data.picture,
            encrypted_password: data.encrypted_password,
            deleted_at: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.collection
            .insert_one(&document)
            .await
            .map_err(|e| UserError::Unknown(e.into()))?;

        Ok(document)
    }

    async fn update_by_id(
        &self,
        id: ObjectId,
        data: UpdateUserData,
    ) -> Result<Option<UserEntity>, UserError> {
        let mut set = doc! {};

        if let Some(full_name) = data.full_name {
            set.insert("full_name", full_name);
        }
        if let Some(username) = data.username {
            set.insert("username", username);
        }
        if let Some(picture) = data.picture {
            set.insert("picture", picture);
        }
        if let Some(language) = data.language {
            set.insert("language", language);
        }
        if let Some(regions) = data.regions {
            set.insert("regions", regions);
        }
        if let Some(currency) = data.currency {
            set.insert("currency", currency);
        }

        let document = self
            .collection
            .find_one_and_update(doc! { "_id": id, "deletedAt": null }, doc! { "$set": set })
            .return_document(ReturnDocument::After)
            .await
            .map_err(|e| UserError::Unknown(e.into()))?;

        Ok(document)
    }

    async fn soft_delete_by_id(&self, id: ObjectId) -> Result<(), UserError> {
        let user = self.find_by_id(id).await?.ok_or(UserError::NotFound)?;

        let prefix = nanoid::nanoid!(10);
        let deleted_email = format!("{}_{}", prefix, user.email);
        let deleted_username = format!("{}_{}", prefix, user.username);

        self.collection
            .update_one(
                doc! { "_id": id },
                doc! {
                    "$set": {
                        "email": deleted_email,
                        "username": deleted_username,
                        "deletedAt": chrono::Utc::now()
                    }
                },
            )
            .await
            .map_err(|e| UserError::Unknown(e.into()))?;

        Ok(())
    }
}
