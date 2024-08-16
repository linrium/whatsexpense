use bson::oid::ObjectId;
use serde::Deserialize;
use utoipa::IntoParams;

use crate::common::mongo::Cursor;

pub struct ListMessagesData {
    pub user_id: ObjectId,
    pub cursor: Option<Cursor>,
}

pub struct ListMessagesInput {
    pub user_id: ObjectId,
    pub cursor: Option<Cursor>,
}

#[derive(Deserialize, IntoParams)]
pub struct ListMessagesQuery {
    pub after: Option<String>,
    pub limit: Option<i64>,
}
