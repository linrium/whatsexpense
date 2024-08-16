use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PresignGetPayload {
    #[schema(example = "https://example.com/presigned/image.jpg")]
    pub url: String,
}
