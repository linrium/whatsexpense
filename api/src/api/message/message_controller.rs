use std::str::FromStr;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use bson::oid::ObjectId;
use utoipa::OpenApi;

use crate::api::infer::models::InferMode;
use crate::api::infer::InferOptions;
use crate::api::message::{
    CreateMessageBody, CreateMessageInput, ListMessagesInput, ListMessagesQuery, Message,
    MessageError,
};
use crate::api::state::AppState;
use crate::api::transaction::Transaction;
use crate::api::user::User;
use crate::common::errors::AppError;
use crate::common::hooks::ValidJson;
use crate::common::mongo::{Cursor, FindOptions};
use crate::object_id;

#[utoipa::path(
    get,
    path = "",
    params(
        ListMessagesQuery,
    ),
    responses(
        (status = 200, description = "List messages successfully", body = [Message]),
    )
)]
pub async fn list_messages(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Query(query): Query<ListMessagesQuery>,
) -> Result<Json<Vec<Message>>, AppError> {
    let messages = state
        .message_service
        .list(
            ListMessagesInput {
                user_id: object_id!(&user.id),
                cursor: query.after.map(Cursor::try_from).transpose()?,
            },
            FindOptions {
                limit: query.limit,
                skip: None,
            },
        )
        .await?;

    Ok(Json(messages))
}

#[utoipa::path(
    post,
    path = "",
    request_body = CreateMessageBody,
    responses(
        (status = 200, description = "Create message successfully", body = [Message]),
    )
)]
pub async fn create_message(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    ValidJson(body): ValidJson<CreateMessageBody>,
) -> Result<Json<Vec<Message>>, AppError> {
    let categories = state.category_service.find().await?;
    let infer_service = state.infer_service_factory.create_service(InferMode::Text);

    let (invoice_tool, completion) = infer_service
        .infer(
            body.content.clone(),
            InferOptions {
                currencies: vec![user.currency.clone()],
                categories,
            },
        )
        .await?;

    let messages = state
        .message_service
        .create(CreateMessageInput {
            prompt: body.content,
            currencies: vec![user.currency],
            user_id: object_id!(&user.id),
            invoice_tool,
            completion,
            media_path: None,
            media_type: None,
        })
        .await?;

    Ok(Json(messages))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    responses(
        (status = 204, description = "Delete message successfully"),
    ),
    params(
        ("id" = ObjectId, Path, description = "Message database id to delete message for"),
    )
)]
pub async fn delete_message(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path((id,)): Path<(String,)>,
) -> Result<StatusCode, AppError> {
    let id = ObjectId::from_str(&id).map_err(|e| AppError::Unknown(e.into()))?;

    state
        .message_service
        .delete_many_by_id(id, object_id!(&user.id))
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/{id}/transactions",
    responses(
        (status = 200, description = "List transactions by message id successfully", body = [Transaction]),
    ),
    params(
        ("id" = ObjectId, Path, description = "Message database id to get transactions for"),
    )
)]
pub async fn list_transactions(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path((id,)): Path<(String,)>,
) -> Result<Json<Vec<Transaction>>, AppError> {
    let message = state.message_service.find_by_id(object_id!(&id)).await?;
    if let Some(message) = message {
        if message.to_id != user.id {
            return Err(AppError::Forbidden);
        }
    } else {
        return Err(MessageError::NotFound.into());
    }

    let transactions = state
        .transaction_service
        .find_by_message_id(object_id!(&id))
        .await?;

    Ok(Json(transactions))
}

#[derive(OpenApi)]
#[openapi(
    paths(list_messages, create_message, delete_message, list_transactions),
    components(
        schemas(
            CreateMessageBody,
            Message,
            Transaction,
        )
    ),
    tags(
        (name = "crate::api::message", description = "Message API")
    )
)]
pub struct MessageApiDoc;
