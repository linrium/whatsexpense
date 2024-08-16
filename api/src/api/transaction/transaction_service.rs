use std::sync::Arc;

use async_trait::async_trait;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::ClientSession;

use crate::api::transaction::*;
use crate::common::errors::AppError;

#[async_trait]
pub trait TransactionServiceExt: Send + Sync {
    async fn insert_one(&self, data: InsertTransactionInput) -> Result<Transaction, AppError>;
    async fn insert_many_with_session(
        &self,
        items: &Vec<InsertTransactionInput>,
        session: &mut ClientSession,
    ) -> Result<Vec<Transaction>, AppError>;
    async fn find_by_message_id(&self, message_id: ObjectId) -> Result<Vec<Transaction>, AppError>;
    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Transaction>, AppError>;
    async fn find_by_invoice_id(&self, invoice_id: ObjectId) -> Result<Vec<Transaction>, AppError>;
    async fn update_by_id(
        &self,
        id: ObjectId,
        data: UpdateTransactionData,
    ) -> Result<Option<Transaction>, AppError>;
    async fn update_many(&self, input: Vec<UpdateTransactionInput>) -> Result<bool, AppError>;
    async fn delete_many_by_ids(
        &self,
        ids: &[ObjectId],
        user_id: &ObjectId,
    ) -> Result<bool, AppError>;
    async fn delete_many_by_ids_with_session(
        &self,
        ids: &[ObjectId],
        session: &mut ClientSession,
    ) -> Result<bool, AppError>;
}

pub type TransactionServiceDyn = Arc<dyn TransactionServiceExt + Send + Sync>;

#[derive(Clone)]
pub struct TransactionService {
    pub repo: TransactionRepoDyn,
}

#[async_trait]
impl TransactionServiceExt for TransactionService {
    async fn insert_one(&self, data: InsertTransactionInput) -> Result<Transaction, AppError> {
        self.repo
            .insert_one(data.into())
            .await
            .map(Into::into)
            .map_err(|e| e.into())
    }

    async fn insert_many_with_session(
        &self,
        items: &Vec<InsertTransactionInput>,
        session: &mut ClientSession,
    ) -> Result<Vec<Transaction>, AppError> {
        let items = items
            .into_iter()
            .map(|item| item.into())
            .collect::<Vec<InsertTransactionData>>();

        self.repo
            .insert_many_with_session(&items, session)
            .await
            .map(|items| items.into_iter().map(Into::into).collect())
            .map_err(|e| e.into())
    }

    async fn find_by_message_id(&self, message_id: ObjectId) -> Result<Vec<Transaction>, AppError> {
        self.repo
            .find(doc! { "messageId": message_id })
            .await
            .map(|items| items.into_iter().map(Transaction::from).collect())
            .map_err(|e| e.into())
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Transaction>, AppError> {
        self.repo
            .find_by_id(id)
            .await
            .map(|v| v.map(Into::into))
            .map_err(|e| e.into())
    }

    async fn find_by_invoice_id(&self, invoice_id: ObjectId) -> Result<Vec<Transaction>, AppError> {
        self.repo
            .find_by_invoice_id(invoice_id)
            .await
            .map(|items| items.into_iter().map(Into::into).collect())
            .map_err(|e| e.into())
    }

    async fn update_by_id(
        &self,
        id: ObjectId,
        data: UpdateTransactionData,
    ) -> Result<Option<Transaction>, AppError> {
        self.repo
            .update_by_id(id, data)
            .await
            .map(|v| v.map(Into::into))
            .map_err(|e| e.into())
    }

    async fn update_many(&self, input: Vec<UpdateTransactionInput>) -> Result<bool, AppError> {
        self.repo
            .update_many(
                input
                    .into_iter()
                    .map(|v| UpdateTransactionData {
                        id: v.id,
                        user_id: v.user_id,
                        amount: v.amount,
                        currency: v.currency,
                        category_id: v.category_id,
                        type_: v.type_,
                        unit: v.unit,
                        quantity: v.quantity,
                        issued_at: v.issued_at,
                        title: v.title,
                    })
                    .collect(),
            )
            .await
            .map_err(|e| e.into())
    }

    async fn delete_many_by_ids(
        &self,
        ids: &[ObjectId],
        user_id: &ObjectId,
    ) -> Result<bool, AppError> {
        self.repo
            .delete_many_by_ids(ids, user_id)
            .await
            .map_err(|e| e.into())
    }

    async fn delete_many_by_ids_with_session(
        &self,
        ids: &[ObjectId],
        session: &mut ClientSession,
    ) -> Result<bool, AppError> {
        self.repo
            .delete_many_by_ids_with_session(ids, session)
            .await
            .map_err(|e| e.into())
    }
}
