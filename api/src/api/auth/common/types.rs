use serde::{Deserialize, Serialize};
use strum::EnumString;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, EnumString, ToSchema)]
pub enum Provider {
    Apple,
    Google,
    Password,
}
