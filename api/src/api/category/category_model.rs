use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    #[schema(example = "housing")]
    pub id: String,
    #[schema(example = "Housing")]
    pub name: String,
    #[schema(example = "Rent, mortgage, property tax, etc.")]
    pub description: String,
    #[schema(example = "#fdaaaa")]
    pub color: String,
    #[schema(example = "outcome")]
    pub r#type: String,
}

impl Category {
    pub const fn new(
        id: String,
        name: String,
        description: String,
        color: String,
        r#type: String,
    ) -> Self {
        Self {
            id,
            name,
            description,
            color,
            r#type,
        }
    }
}
