use crate::api::transaction::*;
use async_trait::async_trait;
use bson::oid::ObjectId;
use bson::{doc, Document};
use futures::StreamExt;
use mongodb::options::ReturnDocument;
use mongodb::{ClientSession, Collection};
use std::sync::Arc;

#[async_trait]
pub trait TransactionRepoExt: Send + Sync {
    async fn insert_one(
        &self,
        data: InsertTransactionData,
    ) -> Result<TransactionEntity, TransactionError>;
    async fn insert_many_with_session(
        &self,
        items: &Vec<InsertTransactionData>,
        session: &mut ClientSession,
    ) -> Result<Vec<TransactionEntity>, TransactionError>;
    async fn find(&self, filter: Document) -> Result<Vec<TransactionEntity>, TransactionError>;
    async fn find_by_id(&self, id: ObjectId)
        -> Result<Option<TransactionEntity>, TransactionError>;
    async fn find_by_invoice_id(
        &self,
        invoice_id: ObjectId,
    ) -> Result<Vec<TransactionEntity>, TransactionError>;
    async fn update_many(&self, data: Vec<UpdateTransactionData>)
        -> Result<bool, TransactionError>;
    async fn update_by_id(
        &self,
        id: ObjectId,
        data: UpdateTransactionData,
    ) -> Result<Option<TransactionEntity>, TransactionError>;
    async fn delete_many_by_ids(
        &self,
        ids: &[ObjectId],
        user_id: &ObjectId,
    ) -> Result<bool, TransactionError>;
    async fn delete_many_by_ids_with_session(
        &self,
        ids: &[ObjectId],
        session: &mut ClientSession,
    ) -> Result<bool, TransactionError>;
}

pub type TransactionRepoDyn = Arc<dyn TransactionRepoExt + Send + Sync>;

#[derive(Clone)]
pub struct TransactionRepo {
    pub collection: Collection<TransactionEntity>,
}

#[async_trait]
impl TransactionRepoExt for TransactionRepo {
    async fn insert_one(
        &self,
        data: InsertTransactionData,
    ) -> Result<TransactionEntity, TransactionError> {
        let document = TransactionEntity {
            id: ObjectId::new(),
            message_id: data.message_id,
            user_id: data.user_id,
            invoice_id: data.invoice_id,
            title: data.title,
            amount: data.amount,
            currency: data.currency,
            category_id: data.category_id,
            r#type: data.r#type,
            unit: data.unit,
            quantity: data.quantity,
            issued_at: data.issued_at,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.collection
            .insert_one(&document)
            .await
            .map_err(|e| TransactionError::Unknown(e.into()))?;

        Ok(document)
    }

    async fn insert_many_with_session(
        &self,
        items: &Vec<InsertTransactionData>,
        session: &mut ClientSession,
    ) -> Result<Vec<TransactionEntity>, TransactionError> {
        let mut now = chrono::Utc::now();
        let mut documents = vec![];
        for item in items {
            let document = TransactionEntity {
                id: ObjectId::new(),
                message_id: item.message_id,
                user_id: item.user_id,
                invoice_id: item.invoice_id,
                title: item.title.clone(),
                amount: item.amount,
                currency: item.currency.clone(),
                category_id: item.category_id.clone(),
                r#type: item.r#type.clone(),
                unit: item.unit.clone(),
                quantity: item.quantity,
                issued_at: item.issued_at,
                created_at: now,
                updated_at: now,
            };

            now = now + chrono::Duration::seconds(1);
            documents.push(document);
        }

        self.collection
            .insert_many(&documents)
            .session(session)
            .await
            .map_err(|e| TransactionError::Unknown(e.into()))?;

        Ok(documents)
    }

    async fn find(&self, filter: Document) -> Result<Vec<TransactionEntity>, TransactionError> {
        let mut cursor = self
            .collection
            .find(filter)
            .await
            .map_err(|e| TransactionError::Unknown(e.into()))?;

        let mut documents = vec![];
        while let Some(Ok(document)) = cursor.next().await {
            documents.push(document);
        }

        Ok(documents)
    }

    async fn find_by_id(
        &self,
        id: ObjectId,
    ) -> Result<Option<TransactionEntity>, TransactionError> {
        self.collection
            .find_one(doc! { "id": id })
            .await
            .map_err(|_| TransactionError::NotFound)
    }

    async fn find_by_invoice_id(
        &self,
        invoice_id: ObjectId,
    ) -> Result<Vec<TransactionEntity>, TransactionError> {
        let mut cursor = self
            .collection
            .find(doc! { "invoiceId": invoice_id })
            .await
            .map_err(|_| TransactionError::NotFound)?;

        let mut documents = vec![];
        while let Some(Ok(transaction)) = cursor.next().await {
            documents.push(transaction);
        }

        Ok(documents)
    }

    async fn update_many(
        &self,
        data: Vec<UpdateTransactionData>,
    ) -> Result<bool, TransactionError> {
        for item in data {
            self.update_by_id(item.id, item).await?;
        }

        Ok(true)
    }

    async fn update_by_id(
        &self,
        id: ObjectId,
        data: UpdateTransactionData,
    ) -> Result<Option<TransactionEntity>, TransactionError> {
        let mut set = doc! {};
        if let Some(amount) = data.amount {
            set.insert("amount", amount);
        }
        if let Some(currency) = data.currency {
            set.insert("currency", currency);
        }
        if let Some(category_id) = data.category_id {
            set.insert("category_id", category_id);
        }
        if let Some(type_) = data.type_ {
            set.insert("type", type_);
        }
        if let Some(unit) = data.unit {
            set.insert("unit", unit);
        }
        if let Some(quantity) = data.quantity {
            set.insert("quantity", quantity);
        }
        if let Some(issued_at) = data.issued_at {
            set.insert("issued_at", issued_at);
        }
        if let Some(title) = data.title {
            set.insert("title", title);
        }

        let document = self
            .collection
            .find_one_and_update(
                doc! { "_id": id, "userId": data.user_id },
                doc! { "$set": set },
            )
            .return_document(ReturnDocument::After)
            .await
            .map_err(|e| TransactionError::Unknown(e.into()))?;

        Ok(document)
    }

    async fn delete_many_by_ids(
        &self,
        ids: &[ObjectId],
        user_id: &ObjectId,
    ) -> Result<bool, TransactionError> {
        self.collection
            .delete_many(doc! { "_id": { "$in": ids }, "userId": user_id })
            .await
            .map(|v| v.deleted_count == ids.len() as u64)
            .map_err(|e| TransactionError::Unknown(e.into()))
    }

    async fn delete_many_by_ids_with_session(
        &self,
        ids: &[ObjectId],
        session: &mut ClientSession,
    ) -> Result<bool, TransactionError> {
        self.collection
            .delete_many(doc! { "_id": { "$in": ids } })
            .session(session)
            .await
            .map(|v| v.deleted_count == ids.len() as u64)
            .map_err(|e| TransactionError::Unknown(e.into()))
    }
}
