use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct SignUpWithPasswordBody {
    #[schema(example = "test@test.com")]
    pub email: String,
    #[schema(example = "123456")]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SignUpWithPasswordInput {
    pub email: String,
    pub password: String,
    pub picture: Option<String>,
    pub username: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
}
