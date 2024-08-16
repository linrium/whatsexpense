use bson::oid::ObjectId;

pub struct InsertTransactionInput {
    pub message_id: ObjectId,
    pub user_id: ObjectId,
    pub invoice_id: ObjectId,

    pub title: String,
    pub amount: f64,
    pub currency: String,
    pub category_id: String,
    pub r#type: String,
    pub unit: Option<String>,
    pub quantity: f64,
    pub issued_at: chrono::DateTime<chrono::Utc>,
}

pub struct InsertTransactionData {
    pub message_id: ObjectId,
    pub user_id: ObjectId,
    pub invoice_id: ObjectId,

    pub title: String,
    pub amount: f64,
    pub currency: String,
    pub category_id: String,
    pub r#type: String,
    pub unit: Option<String>,
    pub quantity: f64,
    pub issued_at: chrono::DateTime<chrono::Utc>,
}

impl From<InsertTransactionInput> for InsertTransactionData {
    fn from(value: InsertTransactionInput) -> Self {
        Self {
            message_id: value.message_id,
            user_id: value.user_id,
            invoice_id: value.invoice_id,
            title: value.title,
            amount: value.amount,
            currency: value.currency,
            category_id: value.category_id,
            r#type: value.r#type,
            unit: value.unit,
            quantity: value.quantity,
            issued_at: value.issued_at,
        }
    }
}

impl From<&InsertTransactionInput> for InsertTransactionData {
    fn from(value: &InsertTransactionInput) -> Self {
        Self {
            message_id: value.message_id,
            user_id: value.user_id,
            title: value.title.clone(),
            amount: value.amount,
            currency: value.currency.clone(),
            category_id: value.category_id.clone(),
            r#type: value.r#type.clone(),
            invoice_id: value.invoice_id,
            unit: value.unit.clone(),
            quantity: value.quantity,
            issued_at: value.issued_at,
        }
    }
}
