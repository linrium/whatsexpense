use async_openai::types::{FunctionObject, FunctionObjectArgs};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct InvoiceToolRaw {
    pub timestamp: Option<String>,
    #[serde(default)]
    pub purchased_items: Vec<PurchasedItemToolRaw>,
    #[serde(default)]
    pub discounts: Vec<DiscountToolRaw>,
    #[serde(default)]
    pub taxes: Vec<TaxToolRaw>,
    pub subtotal: Option<f64>,
    pub total: f64,
    pub currency: String,
    pub card_number: Option<i16>,
}

#[derive(Deserialize, Debug)]
pub struct PurchasedItemToolRaw {
    pub title: String,
    pub quantity: f64,
    pub amount: f64,
    pub unit: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct DiscountToolRaw {
    pub discount_for_item: String,
    pub discount_rate: f32,
    pub discount_amount: f32,
}

#[derive(Deserialize, Debug)]
pub struct TaxToolRaw {
    pub tax_rate: f32,
    pub tax_amount: f32,
}

pub fn make_infer_invoice_tool(currencies: Vec<String>) -> FunctionObject {
    FunctionObjectArgs::default()
        .name("infer_invoice_and_purchased_items")
        .description("Infer invoice details from a given prompt")
        .parameters(json!({
            "type": "object",
            "required": ["purchased_items", "total"],
            "properties": {
                "timestamp": {
                    "type": "string",
                    "description": "The issued date of the invoice in the format ISO 8601 e.g. 2022-01-01T00:00:00Z"
                },
                "purchased_items": {
                    "type": "array",
                    "description": "The purchased items in the invoice including name, quantity and amount",
                    "items": {
                        "type": "object",
                        "required": [
                            "title",
                            "amount"
                        ],
                        "properties": {
                            "title": {
                                "type": "string",
                                "description": "The name of the purchased item in the original language"
                            },
                            "quantity": {
                                "type": "number",
                                "description": "The quantity of the purchased item. It usually less than 20"
                            },
                            "amount": {
                                "type": "number",
                                "description": "The total of the item"
                            },
                            "unit": {
                                "type": "string",
                                "description": "The unit of the quantity e.g. item, kg, liter, etc."
                            }
                        }
                    }
                },
                "discounts": {
                "type": "array",
                "description": "The discounts in the invoice. Ignore if there is no discount",
                "items": {
                    "type": "object",
                    "properties": {
                        "discount_for_item": {
                            "type": "string",
                            "description": "The item's name that the discount is applied to. It placed next above the current line"
                        },
                        "discount_rate": {
                            "type": "number",
                            "description": "The rate of the discount e.g. 8, 10"
                        },
                        "discount_amount": {
                            "type": "number",
                            "description": "The amount of the discount"
                        }
                    }
                }
            },
            "taxes": {
                "type": "array",
                "description": "The taxes of the invoice. Ignore if there is no tax",
                "items": {
                    "type": "object",
                    "properties": {
                        "tax_rate": {
                            "type": "number",
                            "description": "The rate of the tax e.g. 8, 10"
                        },
                        "tax_amount": {
                            "type": "number",
                            "description": "The amount of the tax"
                        }
                    }
                }
            },
            "subtotal": {
                "type": "number",
                "description": "The subtotal of the invoice not including taxes"
            },
            "total": {
                "type": "number",
                "description": "The total of the invoice including taxes"
            },
            "currency": {
                "type": "string",
                "enum": currencies,
                "description": "The currency based on the main language in the invoice",
            },
            "card_number": {
                "type": "number",
                "description": "The last 4 digits of the card number used to pay the invoice"
            }
            },

        }))
        .build()
        .unwrap()
}
