use crate::api::state::AppState;
use crate::api::user::{UpdateUserBody, UpdateUserInput, User, UserError};
use crate::common::errors::AppError;
use crate::common::hooks::ValidJson;
use crate::object_id;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use utoipa::OpenApi;

#[utoipa::path(
    get,
    path = "/me",
    responses(
        (status = 200, description = "Get message successfully", body = User),
    ),
)]
pub(crate) async fn get_user(Extension(user): Extension<User>) -> Result<Json<User>, AppError> {
    Ok(Json(user))
}

#[utoipa::path(
    patch,
    path = "/me",
    request_body = UpdateUserBody,
    responses(
        (status = 200, description = "Update message successfully", body = User),
    ),
)]
pub(crate) async fn update_user(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    ValidJson(body): ValidJson<UpdateUserBody>,
) -> Result<Json<User>, AppError> {
    let user = state
        .user_service
        .update_by_id(
            object_id!(&user.id),
            UpdateUserInput {
                full_name: body.full_name,
                username: body.username,
                picture: body.picture,
                language: body.language,
                regions: body.regions,
                currency: body.currency,
            },
        )
        .await?;

    if let Some(user) = user {
        return Ok(Json(user));
    }

    Err(UserError::NotFound.into())
}

#[utoipa::path(
    delete,
    path = "/me",
    responses(
        (status = 204, description = "Delete user successfully"),
    ),
)]
pub async fn delete_user(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<StatusCode, AppError> {
    state
        .user_service
        .soft_delete_by_id(object_id!(&user.id))
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(OpenApi)]
#[openapi(
    paths(get_user, update_user, delete_user),
    components(
        schemas(
            UpdateUserBody,
            User,
        )
    ),
    tags(
        (name = "crate::api::user", description = "User API")
    )
)]
pub struct UserApiDoc;
