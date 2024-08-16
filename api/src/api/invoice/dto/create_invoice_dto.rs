use crate::api::invoice::{Discount, Tax};
use bson::oid::ObjectId;

pub struct CreateInvoiceData {
    pub id: ObjectId,
    pub user_id: ObjectId,
    pub message_id: ObjectId,
    pub taxes: Vec<Tax>,
    pub discounts: Vec<Discount>,
    pub subtotal: Option<f64>,
    pub total: f64,
    pub currency: String,
    pub card_number: Option<i16>,
    pub media_path: Option<String>,
    pub media_type: Option<String>,
}

pub struct CreateInvoiceInput {
    pub id: ObjectId,
    pub user_id: ObjectId,
    pub message_id: ObjectId,
    pub taxes: Vec<Tax>,
    pub discounts: Vec<Discount>,
    pub subtotal: Option<f64>,
    pub total: f64,
    pub currency: String,
    pub card_number: Option<i16>,
    pub media_path: Option<String>,
    pub media_type: Option<String>,
    pub issued_at: chrono::DateTime<chrono::Utc>,
}
