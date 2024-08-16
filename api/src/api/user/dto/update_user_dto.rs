use serde::Deserialize;
#[allow(unused_imports)]
use serde_json::json;
use utoipa::ToSchema;
use validator::Validate;

pub struct UpdateUserData {
    pub full_name: Option<String>,
    pub username: Option<String>,
    pub picture: Option<String>,
    pub language: Option<String>,
    pub regions: Option<Vec<String>>,
    pub currency: Option<String>,
}

pub struct UpdateUserInput {
    pub language: Option<String>,
    pub regions: Option<Vec<String>>,
    pub currency: Option<String>,
    pub full_name: Option<String>,
    pub username: Option<String>,
    pub picture: Option<String>,
}

impl From<UpdateUserInput> for UpdateUserData {
    fn from(input: UpdateUserInput) -> Self {
        Self {
            full_name: input.full_name,
            username: input.username,
            picture: input.picture,
            language: input.language,
            regions: input.regions,
            currency: input.currency,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct UpdateUserBody {
    #[schema(example = "en")]
    pub language: Option<String>,
    #[schema(example = json!(["us", "ca"]))]
    pub regions: Option<Vec<String>>,
    #[schema(example = "USD")]
    pub currency: Option<String>,
    #[schema(example = "John Doe")]
    pub full_name: Option<String>,
    #[schema(example = "johndoe")]
    pub username: Option<String>,
    #[schema(example = "https://example.com/johndoe.png")]
    pub picture: Option<String>,
}
