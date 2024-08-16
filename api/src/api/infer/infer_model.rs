use crate::api::infer::constants::tools::{
    parse_amount_string, parse_issued_at_string, InvoiceToolRaw, TransactionToolRaw,
};
use crate::api::infer::tools::{
    CategoryToolRaw, DiscountToolRaw, PurchasedItemToolRaw, TaxToolRaw,
};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum InferMode {
    Text,
    Invoice,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CategoryTool {
    pub category_id: String,
    pub r#type: String,
}

impl Default for CategoryTool {
    fn default() -> Self {
        Self {
            category_id: "unknown".to_string(),
            r#type: "outcome".to_string(),
        }
    }
}

impl From<CategoryToolRaw> for CategoryTool {
    fn from(raw: CategoryToolRaw) -> Self {
        Self {
            category_id: raw.category,
            r#type: raw.r#type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceTool {
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub transactions: Vec<TransactionTool>,
    pub discounts: Vec<DiscountTool>,
    pub taxes: Vec<TaxTool>,
    pub subtotal: Option<f64>,
    pub total: f64,
    pub currency: String,
    pub card_number: Option<i16>,
}

impl From<InvoiceToolRaw> for InvoiceTool {
    fn from(raw: InvoiceToolRaw) -> Self {
        Self {
            issued_at: parse_issued_at_string(chrono::Utc::now(), raw.timestamp),
            transactions: raw
                .purchased_items
                .into_iter()
                .map(|item| item.into())
                .collect(),
            discounts: raw
                .discounts
                .into_iter()
                .map(|item| DiscountTool::from(item))
                .filter(|item| item.amount != 0.0)
                .collect(),
            taxes: raw
                .taxes
                .into_iter()
                .map(|item| TaxTool::from(item))
                .filter(|item| item.amount != 0.0)
                .collect(),
            subtotal: raw.subtotal,
            total: raw.total,
            currency: raw.currency,
            card_number: raw.card_number,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscountTool {
    pub name: String,
    pub rate: f32,
    pub amount: f32,
}

impl From<DiscountToolRaw> for DiscountTool {
    fn from(raw: DiscountToolRaw) -> Self {
        Self {
            name: raw.discount_for_item,
            rate: raw.discount_rate,
            amount: raw.discount_amount,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaxTool {
    pub rate: f32,
    pub amount: f32,
}

impl From<TaxToolRaw> for TaxTool {
    fn from(raw: TaxToolRaw) -> Self {
        Self {
            rate: raw.tax_rate,
            amount: raw.tax_amount,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionTool {
    pub title: String,
    pub currency: String,
    pub category_id: String,
    pub r#type: String,
    pub amount: f64,
    pub quantity: f64,
    pub unit: Option<String>,
    pub issued_at: chrono::DateTime<chrono::Utc>,
}

impl Default for TransactionTool {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            currency: "USD".to_string(),
            category_id: "unknown".to_string(),
            r#type: "outcome".to_string(),
            amount: 0.0,
            quantity: 1.0,
            unit: None,
            issued_at: chrono::Utc::now(),
        }
    }
}

impl From<TransactionToolRaw> for TransactionTool {
    fn from(raw: TransactionToolRaw) -> Self {
        let now = chrono::Utc::now();
        Self {
            title: raw.title,
            currency: raw.currency,
            amount: parse_amount_string(raw.amount),
            quantity: raw.quantity.unwrap_or(1.0),
            unit: raw.unit,
            issued_at: parse_issued_at_string(now, raw.date),
            ..Default::default()
        }
    }
}

impl From<PurchasedItemToolRaw> for TransactionTool {
    fn from(raw: PurchasedItemToolRaw) -> Self {
        Self {
            title: raw.title,
            currency: "USD".to_string(),
            amount: raw.amount,
            quantity: raw.quantity,
            unit: raw.unit,
            issued_at: chrono::Utc::now(),
            ..Default::default()
        }
    }
}
