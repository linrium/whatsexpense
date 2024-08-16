use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::api::invoice::InvoiceEntity;
use crate::api::transaction::TransactionEntity;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MessageEntity {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub content: String,
    pub from_id: ObjectId,
    pub to_id: ObjectId,
    pub thread_id: ObjectId,
    pub reply_to_id: Option<ObjectId>,
    pub completion: Option<String>,
    #[serde(skip_serializing)]
    pub invoice: Option<InvoiceEntity>,
    #[serde(skip_serializing)]
    pub transactions: Option<Vec<TransactionEntity>>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
