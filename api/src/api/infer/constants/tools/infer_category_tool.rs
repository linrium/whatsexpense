use crate::api::category::Category;
use async_openai::types::{FunctionObject, FunctionObjectArgs};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryToolRaw {
    pub category: String,
    pub r#type: String,
}

pub fn make_infer_category_tool(categories: Vec<Category>) -> FunctionObject {
    let categories = categories
        .into_iter()
        .map(|category| category.id)
        .collect::<Vec<String>>();

    FunctionObjectArgs::default()
        .name("infer_transactions_category")
        .description("Get the category of the transaction in predefined categories")
        .parameters(json!({
            "type": "object",
            "required": ["category", "type"],
            "properties": {
                "category": {
                    "type": "string",
                    "enum": categories,
                    "description": "The category of the transaction",
                },
                "type": {
                    "type": "string",
                    "enum": ["income", "outcome", "debt", "other"],
                    "description": "The type of the transaction",
                },
            },
        }))
        .build()
        .unwrap()
}
