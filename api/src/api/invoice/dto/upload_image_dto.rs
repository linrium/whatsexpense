use bson::oid::ObjectId;
use serde::Deserialize;
use utoipa::ToSchema;

#[allow(unused)]
#[derive(Deserialize, ToSchema)]
pub struct UploadImageBody {
    #[schema(value_type = String, format = Binary)]
    pub file: Vec<u8>,
}

pub struct UploadImageInput {
    pub user_id: ObjectId,
    pub content_type: String,
}

pub struct UploadedImage {
    pub path: String,
    pub content_type: String,
    pub content: String,
}
