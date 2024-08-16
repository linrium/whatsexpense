use serde::Deserialize;
#[allow(unused_imports)]
use serde_json::json;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct DeleteTransactionBody {
    #[schema(example = json!(["5f0e6a1d-d1b0-4a5e-b1e8-e0c4c8d0b2b5"]))]
    pub ids: Vec<String>,
}
