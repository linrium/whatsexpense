use crate::api::invoice::constants::InvoiceError;
use crate::api::invoice::*;
use async_trait::async_trait;
use bson::oid::ObjectId;
use bson::{doc, Document};
use mongodb::{ClientSession, Collection};
use std::sync::Arc;

#[async_trait]
pub trait InvoiceRepoExt: Send + Sync {
    async fn find_one(&self, filter: Document) -> Result<Option<InvoiceEntity>, InvoiceError>;
    async fn find_by_id(&self, id: ObjectId) -> Result<Option<InvoiceEntity>, InvoiceError>;
    async fn insert_one_with_session(
        &self,
        data: CreateInvoiceData,
        session: &mut ClientSession,
    ) -> Result<InvoiceEntity, InvoiceError>;
    async fn delete_by_id_with_session(
        &self,
        id: ObjectId,
        session: &mut ClientSession,
    ) -> Result<bool, InvoiceError>;
}

pub type InvoiceRepoDyn = Arc<dyn InvoiceRepoExt + Send + Sync>;

#[derive(Clone)]
pub struct InvoiceRepo {
    pub collection: Collection<InvoiceEntity>,
}

#[async_trait]
impl InvoiceRepoExt for InvoiceRepo {
    async fn find_one(&self, filter: Document) -> Result<Option<InvoiceEntity>, InvoiceError> {
        self.collection
            .find_one(filter)
            .await
            .map_err(|e| InvoiceError::Unknown(e.into()))
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<Option<InvoiceEntity>, InvoiceError> {
        self.collection
            .find_one(doc! { "_id": id })
            .await
            .map_err(|e| InvoiceError::Unknown(e.into()))
    }

    async fn insert_one_with_session(
        &self,
        data: CreateInvoiceData,
        session: &mut ClientSession,
    ) -> Result<InvoiceEntity, InvoiceError> {
        let document = InvoiceEntity {
            id: data.id,
            user_id: data.user_id,
            message_id: data.message_id,
            taxes: data.taxes,
            discounts: data.discounts,
            subtotal: data.subtotal,
            total: data.total,
            currency: data.currency,
            card_number: data.card_number,
            media_path: data.media_path,
            media_type: data.media_type,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.collection
            .insert_one(&document)
            .session(session)
            .await
            .map_err(|e| InvoiceError::Unknown(e.into()))?;

        Ok(document)
    }

    async fn delete_by_id_with_session(
        &self,
        id: ObjectId,
        session: &mut ClientSession,
    ) -> Result<bool, InvoiceError> {
        self.collection
            .delete_one(doc! { "_id": id })
            .session(session)
            .await
            .map(|v| v.deleted_count > 0)
            .map_err(|e| InvoiceError::Unknown(e.into()))
    }
}
