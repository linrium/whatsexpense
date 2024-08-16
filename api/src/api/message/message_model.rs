use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::json;
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use utoipa::ToSchema;

use crate::api::invoice::Invoice;
use crate::api::message::MessageEntity;
use crate::api::transaction::Transaction;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[schema(example = "669e5f02b781150b9a578205")]
    pub id: String,
    #[schema(example = "Hello, world!")]
    pub content: String,
    #[schema(example = "6693360a1bcf9a76a63f4cfd")]
    pub from_id: String,
    #[schema(example = "66990b1947d76ec3781adc9d")]
    pub to_id: String,
    #[schema(example = "66b398530dc5bdb569fbc83b")]
    pub thread_id: String,
    #[schema(example = "669e5f02b781150b9a578204")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub reply_to_id: Option<String>,
    pub invoice: Option<Invoice>,
    pub transactions: Option<Vec<Transaction>>,
    #[schema(example = "2024-07-22T13:30:42.246014Z")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[schema(example = "2024-07-22T13:30:42.246017Z")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<MessageEntity> for Message {
    fn from(value: MessageEntity) -> Self {
        Self {
            id: value.id.to_hex(),
            content: value.content,
            from_id: value.from_id.to_hex(),
            to_id: value.to_id.to_hex(),
            thread_id: value.thread_id.to_hex(),
            reply_to_id: value.reply_to_id.map(|id| id.to_hex()),
            invoice: value.invoice.map(Into::into),
            transactions: value
                .transactions
                .map(|txs| txs.into_iter().map(Into::into).collect()),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
