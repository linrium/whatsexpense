use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct SignInWithPasswordBody {
    #[schema(example = "test@test.com")]
    pub email: String,
    #[schema(example = "123456")]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SignInWithPasswordInput {
    pub email: String,
    pub password: String,
}
