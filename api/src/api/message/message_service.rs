use crate::api::invoice::{CreateInvoiceInput, InvoiceServiceDyn};
use crate::api::message::*;
use crate::api::transaction::{InsertTransactionInput, TransactionServiceDyn};
use crate::common::errors::AppError;
use crate::common::mongo::FindOptions;
use crate::object_id;
use async_trait::async_trait;
use bson::doc;
use bson::oid::ObjectId;
use futures::FutureExt;
use mongodb::{Client, ClientSession};
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;

#[async_trait]
pub trait MessageServiceExt: Send + Sync {
    async fn list(
        &self,
        data: ListMessagesInput,
        options: FindOptions,
    ) -> Result<Vec<Message>, AppError>;
    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Message>, AppError>;
    async fn insert_one(&self, input: InsertMessageInput) -> Result<Message, AppError>;
    async fn insert_many_with_session(
        &self,
        items: Vec<InsertMessageInput>,
        session: &mut ClientSession,
    ) -> Result<Vec<Message>, AppError>;
    async fn create(&self, input: CreateMessageInput) -> Result<Vec<Message>, AppError>;
    async fn delete_many_by_id(&self, id: ObjectId, user_id: ObjectId) -> Result<bool, AppError>;
}

pub type MessageServiceDyn = Arc<dyn MessageServiceExt + Send + Sync>;

#[derive(Clone)]
pub struct MessageService {
    pub repo: MessageRepoDyn,
    pub mongo_client: Client,
    pub transaction_service: TransactionServiceDyn,
    pub invoice_service: InvoiceServiceDyn,
}

impl MessageService {
    const DEFAULT_BOT_ID: &'static str = "6693360a1bcf9a76a63f4cfd";

    pub fn default_bot_id() -> ObjectId {
        ObjectId::from_str(Self::DEFAULT_BOT_ID).unwrap()
    }
}

#[async_trait]
impl MessageServiceExt for MessageService {
    async fn list(
        &self,
        input: ListMessagesInput,
        options: FindOptions,
    ) -> Result<Vec<Message>, AppError> {
        self.repo
            .list(
                ListMessagesData {
                    user_id: input.user_id,
                    cursor: input.cursor,
                },
                options,
            )
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
            .map_err(|e| e.into())
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Message>, AppError> {
        self.repo
            .find_by_id(id)
            .await
            .map(|v| v.map(Into::into))
            .map_err(Into::into)
    }

    async fn insert_one(&self, input: InsertMessageInput) -> Result<Message, AppError> {
        self.repo
            .insert_one(InsertMessageData {
                id: input.id,
                content: input.content,
                from_id: input.from_id,
                to_id: input.to_id,
                thread_id: input.thread_id,
                reply_to_id: input.reply_to_id,
                completion: input.completion,
                created_at: input.created_at,
            })
            .await
            .map(Into::into)
            .map_err(|e| e.into())
    }

    async fn insert_many_with_session(
        &self,
        items: Vec<InsertMessageInput>,
        session: &mut ClientSession,
    ) -> Result<Vec<Message>, AppError> {
        self.repo
            .insert_many_with_session(
                items
                    .into_iter()
                    .map(|item| InsertMessageData {
                        id: item.id,
                        content: item.content,
                        from_id: item.from_id,
                        to_id: item.to_id,
                        thread_id: item.thread_id,
                        reply_to_id: item.reply_to_id,
                        completion: item.completion,
                        created_at: item.created_at,
                    })
                    .collect(),
                session,
            )
            .await
            .map(|items| items.into_iter().map(Into::into).collect())
            .map_err(|e| e.into())
    }

    async fn create(&self, input: CreateMessageInput) -> Result<Vec<Message>, AppError> {
        let user_message_id = ObjectId::new();
        let bot_message_id = ObjectId::new();
        let invoice_id = ObjectId::new();

        let mut session = self
            .mongo_client
            .start_session()
            .await
            .map_err(|e| AppError::Unknown(e.into()))?;
        let (invoice, messages, txs) = session
            .start_transaction()
            .and_run(&input, |session, input| {
                async move {
                    let invoice_tool = input.invoice_tool.clone();
                    let completion = input.completion.clone();

                    let invoice = self
                        .invoice_service
                        .insert_one_with_session(
                            CreateInvoiceInput {
                                id: invoice_id,
                                user_id: input.user_id.clone(),
                                message_id: bot_message_id,
                                taxes: invoice_tool.taxes.into_iter().map(Into::into).collect(),
                                discounts: invoice_tool
                                    .discounts
                                    .into_iter()
                                    .map(Into::into)
                                    .collect(),
                                subtotal: invoice_tool.subtotal,
                                total: invoice_tool.total,
                                currency: invoice_tool.currency.clone(),
                                card_number: invoice_tool.card_number.clone(),
                                media_path: input.media_path.clone(),
                                media_type: input.media_type.clone(),
                                issued_at: invoice_tool.issued_at,
                            },
                            session,
                        )
                        .await
                        .map_err(|e| mongodb::error::Error::custom(e))?;

                    let messages = self
                        .insert_many_with_session(
                            vec![
                                InsertMessageInput {
                                    id: bot_message_id,
                                    content: "".to_string(),
                                    from_id: Self::default_bot_id(),
                                    to_id: input.user_id,
                                    thread_id: input.user_id,
                                    reply_to_id: Some(user_message_id),
                                    completion: Some(completion.clone()),
                                    created_at: chrono::Utc::now() + chrono::Duration::seconds(1),
                                },
                                InsertMessageInput {
                                    id: user_message_id,
                                    content: input.prompt.clone(),
                                    from_id: input.user_id,
                                    to_id: Self::default_bot_id(),
                                    thread_id: input.user_id,
                                    reply_to_id: None,
                                    completion: None,
                                    created_at: chrono::Utc::now(),
                                },
                            ],
                            session,
                        )
                        .await
                        .map_err(|e| mongodb::error::Error::custom(e))?;

                    let items = invoice_tool
                        .transactions
                        .clone()
                        .into_iter()
                        .map(|tx| InsertTransactionInput {
                            message_id: bot_message_id,
                            user_id: input.user_id.clone(),
                            invoice_id,

                            title: tx.title,
                            amount: tx.amount,
                            currency: tx.currency,
                            category_id: tx.category_id,
                            unit: tx.unit,
                            issued_at: tx.issued_at,
                            quantity: tx.quantity,
                            r#type: tx.r#type,
                        })
                        .collect::<Vec<InsertTransactionInput>>();
                    let transactions = self
                        .transaction_service
                        .insert_many_with_session(&items, session)
                        .await
                        .map_err(|e| mongodb::error::Error::custom(e))?;

                    Ok((invoice, messages, transactions))
                }
                .boxed()
            })
            .await
            .map_err(|e| AppError::Unknown(e.into()))?;

        let mut messages = messages;
        messages[0].invoice = Some(invoice);
        messages[0].transactions = Some(txs);

        Ok(messages)
    }

    async fn delete_many_by_id(&self, id: ObjectId, user_id: ObjectId) -> Result<bool, AppError> {
        let mut session = self
            .mongo_client
            .start_session()
            .await
            .map_err(|e| AppError::Unknown(e.into()))?;

        let message1 = self
            .repo
            .find(
                doc! { "_id": id, "$or": [{ "fromId": user_id },{ "toId": user_id }] },
                FindOptions::with_limit(1),
            )
            .await?
            .first()
            .cloned()
            .ok_or(MessageError::NotFound)?;

        let filter = if message1.from_id == user_id {
            doc! {
                "replyToId": id,
            }
        } else {
            doc! {
                "_id": message1.reply_to_id,
            }
        };
        let message2 = self
            .repo
            .find(filter, FindOptions::with_limit(1))
            .await?
            .first()
            .cloned();

        let messages = if let Some(message2) = message2 {
            vec![message1, message2]
        } else {
            vec![message1]
        };
        let message_has_invoice = messages.iter().find(|m| m.reply_to_id.is_some());
        let (invoice, transactions) = if let Some(message) = message_has_invoice {
            tokio::try_join!(
                self.invoice_service.find_by_message_id(message.id),
                self.transaction_service.find_by_message_id(message.id)
            )?
        } else {
            (None, vec![])
        };

        session
            .start_transaction()
            .and_run(
                (&messages, &invoice, transactions),
                |session, (messages, invoice, transactions)| {
                    async move {
                        let message_ids = messages.iter().map(|m| m.id).collect::<Vec<ObjectId>>();
                        info!(?message_ids, "message ids");
                        self.repo
                            .delete_many_by_ids_with_session(message_ids.as_slice(), session)
                            .await
                            .map_err(|e| mongodb::error::Error::custom(e))?;

                        if let Some(invoice) = invoice {
                            self.invoice_service
                                .delete_by_id_with_session(object_id!(&invoice.id), session)
                                .await
                                .map_err(|e| mongodb::error::Error::custom(e))?;
                        }

                        let transaction_ids = transactions
                            .iter()
                            .map(|t| object_id!(&t.id))
                            .collect::<Vec<ObjectId>>();
                        self.transaction_service
                            .delete_many_by_ids_with_session(transaction_ids.as_slice(), session)
                            .await
                            .map_err(|e| mongodb::error::Error::custom(e))?;

                        Ok(true)
                    }
                    .boxed()
                },
            )
            .await
            .map_err(|e| AppError::Unknown(e.into()))
    }
}
