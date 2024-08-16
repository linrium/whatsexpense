use bson::oid::ObjectId;

pub struct InsertMessageData {
    pub id: ObjectId,
    pub content: String,
    pub from_id: ObjectId,
    pub to_id: ObjectId,
    pub thread_id: ObjectId,
    pub reply_to_id: Option<ObjectId>,
    pub completion: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct InsertMessageInput {
    pub id: ObjectId,
    pub content: String,
    pub from_id: ObjectId,
    pub to_id: ObjectId,
    pub thread_id: ObjectId,
    pub reply_to_id: Option<ObjectId>,
    pub completion: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
