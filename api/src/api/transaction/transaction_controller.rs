use crate::api::state::AppState;
use crate::api::transaction::{
    DeleteTransactionBody, UpdateTransactionBody, UpdateTransactionInput,
};
use crate::api::user::User;
use crate::common::errors::AppError;
use crate::common::hooks::ValidJson;
use crate::object_id;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Extension;
use bson::oid::ObjectId;
use utoipa::OpenApi;

#[utoipa::path(
    patch,
    path = "",
    request_body = [UpdateTransactionBody],
    responses(
        (status = 204, description = "Update transactions by id successfully"),
    ),
)]
pub async fn update_transactions(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    ValidJson(body): ValidJson<Vec<UpdateTransactionBody>>,
) -> Result<StatusCode, AppError> {
    let input = body
        .into_iter()
        .map(|v| UpdateTransactionInput {
            id: object_id!(&v.id),
            user_id: object_id!(&user.id),
            amount: v.amount,
            currency: v.currency,
            title: v.title,
            type_: v.type_,
            unit: v.unit,
            quantity: v.quantity,
            issued_at: v.issued_at,
            category_id: v.category_id,
        })
        .collect();

    state.transaction_service.update_many(input).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    delete,
    path = "",
    request_body = [DeleteTransactionBody],
    responses(
        (status = 204, description = "Delete transactions by id successfully"),
    ),
)]
pub async fn delete_transactions(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    ValidJson(body): ValidJson<DeleteTransactionBody>,
) -> Result<StatusCode, AppError> {
    let ids = body
        .ids
        .into_iter()
        .map(|v| object_id!(&v))
        .collect::<Vec<ObjectId>>();

    state
        .transaction_service
        .delete_many_by_ids(&ids, &object_id!(&user.id))
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(OpenApi)]
#[openapi(
    paths(update_transactions, delete_transactions),
    components(
        schemas(
            UpdateTransactionBody,
        )
    ),
    tags(
        (name = "crate::api::transaction", description = "Transaction API")
    )
)]
pub struct TransactionApiDoc;
