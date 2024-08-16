use crate::api::infer::models::InvoiceTool;
use bson::oid::ObjectId;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateMessageBody {
    #[schema(example = "Buy a cup of coffee 5 USD")]
    pub content: String,
}

pub struct CreateMessageInput {
    pub prompt: String,
    pub currencies: Vec<String>,
    pub user_id: ObjectId,
    pub invoice_tool: InvoiceTool,
    pub completion: String,
    pub media_path: Option<String>,
    pub media_type: Option<String>,
}
