use std::sync::Arc;

use crate::api::message::*;
use crate::common::mongo::FindOptions;
use async_trait::async_trait;
use bson::oid::ObjectId;
use bson::{doc, from_document, Document};
use futures::StreamExt;
use mongodb::{ClientSession, Collection};

#[async_trait]
pub trait MessageRepoExt: Send + Sync {
    async fn list(
        &self,
        data: ListMessagesData,
        options: FindOptions,
    ) -> Result<Vec<MessageEntity>, MessageError>;
    async fn find(
        &self,
        filter: Document,
        options: FindOptions,
    ) -> Result<Vec<MessageEntity>, MessageError>;
    async fn find_by_id(&self, id: ObjectId) -> Result<Option<MessageEntity>, MessageError>;
    async fn insert_one(&self, data: InsertMessageData) -> Result<MessageEntity, MessageError>;
    async fn insert_many_with_session(
        &self,
        items: Vec<InsertMessageData>,
        session: &mut ClientSession,
    ) -> Result<Vec<MessageEntity>, MessageError>;
    async fn delete_many_by_ids_with_session(
        &self,
        ids: &[ObjectId],
        session: &mut ClientSession,
    ) -> Result<bool, MessageError>;
}

pub type MessageRepoDyn = Arc<dyn MessageRepoExt + Send + Sync>;

#[derive(Clone)]
pub struct MessageRepo {
    pub collection: Collection<MessageEntity>,
}

impl MessageRepo {
    const DEFAULT_LIMIT: i64 = 20;
}

#[async_trait]
impl MessageRepoExt for MessageRepo {
    async fn list(
        &self,
        data: ListMessagesData,
        options: FindOptions,
    ) -> Result<Vec<MessageEntity>, MessageError> {
        let limit = options.limit.unwrap_or(Self::DEFAULT_LIMIT);
        let mut filter = doc! {
            "threadId": data.user_id
        };

        if let Some(cursor) = data.cursor {
            let id: ObjectId = cursor.into();
            filter.insert("_id", doc! { "$lt": id });
        }

        let mut cursor = self
            .collection
            .aggregate(vec![
                doc! { "$match": filter },
                doc! {
                    "$lookup": {
                        "from": "invoices",
                        "localField": "_id",
                        "foreignField": "messageId",
                        "as": "invoices"
                    }
                },
                doc! {
                    "$lookup": {
                        "from": "transactions",
                        "localField": "_id",
                        "foreignField": "messageId",
                        "as": "transactions"
                    }
                },
                doc! {
                    "$project": {
                        "_id": 1,
                        "content": 1,
                        "fromId": 1,
                        "toId": 1,
                        "threadId": 1,
                        "replyToId": 1,
                        "createdAt": 1,
                        "updatedAt": 1,
                        "invoice": {
                            "$first": "$invoices"
                        },
                        "transactions": 1
                    }
                },
                doc! {
                    "$sort": { "_id": -1 }
                },
                doc! {
                    "$limit": limit
                },
            ])
            .await
            .map_err(|e| MessageError::Unknown(e.into()))?;

        let mut messages = vec![];
        while let Some(Ok(document)) = cursor.next().await {
            let message: MessageEntity =
                from_document(document).map_err(|e| MessageError::Unknown(e.into()))?;
            messages.push(message);
        }

        Ok(messages)
    }

    async fn find(
        &self,
        filter: Document,
        options: FindOptions,
    ) -> Result<Vec<MessageEntity>, MessageError> {
        let mut cursor = self
            .collection
            .find(filter)
            .limit(options.limit.unwrap_or(Self::DEFAULT_LIMIT))
            .await
            .map_err(|e| MessageError::Unknown(e.into()))?;

        let mut messages = vec![];
        while let Some(Ok(message)) = cursor.next().await {
            messages.push(message);
        }

        Ok(messages)
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<Option<MessageEntity>, MessageError> {
        self.collection
            .find_one(doc! { "_id": id })
            .await
            .map_err(|e| MessageError::Unknown(e.into()))
    }

    async fn insert_one(&self, data: InsertMessageData) -> Result<MessageEntity, MessageError> {
        let document = MessageEntity {
            id: data.id,
            content: data.content,
            from_id: data.from_id,
            to_id: data.to_id,
            thread_id: data.thread_id,
            reply_to_id: data.reply_to_id,
            completion: data.completion,
            invoice: None,
            transactions: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.collection
            .insert_one(&document)
            .await
            .map_err(|e| MessageError::Unknown(e.into()))?;

        Ok(document)
    }

    async fn insert_many_with_session(
        &self,
        items: Vec<InsertMessageData>,
        session: &mut ClientSession,
    ) -> Result<Vec<MessageEntity>, MessageError> {
        let mut documents = vec![];
        for item in items {
            let document = MessageEntity {
                id: item.id,
                content: item.content,
                from_id: item.from_id,
                to_id: item.to_id,
                thread_id: item.thread_id,
                reply_to_id: item.reply_to_id,
                completion: item.completion,
                invoice: None,
                transactions: None,
                created_at: item.created_at,
                updated_at: item.created_at,
            };

            // now = now + chrono::Duration::seconds(1);
            documents.push(document);
        }

        self.collection
            .insert_many(&documents)
            .session(session)
            .await
            .map_err(|e| MessageError::Unknown(e.into()))?;

        Ok(documents)
    }

    async fn delete_many_by_ids_with_session(
        &self,
        ids: &[ObjectId],
        session: &mut ClientSession,
    ) -> Result<bool, MessageError> {
        self.collection
            .delete_many(doc! { "_id": { "$in": ids } })
            .session(session)
            .await
            .map(|v| v.deleted_count == ids.len() as u64)
            .map_err(|e| MessageError::Unknown(e.into()))
    }
}
