use anyhow::anyhow;
use axum::extract::{Query, State};
use axum::{Extension, Json};
use utoipa::OpenApi;

use crate::api::auth::types::Provider;
use crate::api::auth::*;
use crate::api::state::AppState;
use crate::api::user::User;
use crate::common::errors::AppError;
use crate::common::hooks::ValidJson;

#[utoipa::path(
    post,
    path = "/sign-up",
    request_body = SignUpBody,
    responses(
        (status = 200, description = "Sign up successfully", body = AuthPayload),
    )
)]
pub async fn sign_up(
    State(state): State<AppState>,
    ValidJson(body): ValidJson<SignUpWithPasswordBody>,
) -> Result<Json<AuthPayload>, AppError> {
    let resp = state
        .auth_service
        .sign_up_with_password(SignUpWithPasswordInput {
            email: body.email,
            password: body.password,
            picture: None,
            username: None,
            given_name: None,
            family_name: None,
        })
        .await?;

    Ok(Json(resp.into()))
}

#[utoipa::path(
    post,
    path = "/oauth/sign-in",
    request_body = SignInWithOauthBody,
    responses(
        (status = 200, description = "Sign in successfully", body = AuthPayload),
    )
)]
pub async fn sign_in_with_oauth(
    State(state): State<AppState>,
    ValidJson(body): ValidJson<SignInWithOauthBody>,
) -> Result<Json<AuthPayload>, AppError> {
    let resp = match body.provider {
        Provider::Apple => state.auth_service.sign_in_with_apple(body.code).await?,
        Provider::Google => state.auth_service.sign_in_with_google(body.code).await?,
        _ => return Err(AuthError::Unknown(anyhow!("does not support provider")).into()),
    };

    Ok(Json(resp.into()))
}

#[utoipa::path(
    post,
    path = "/sign-in",
    request_body = SignInBody,
    responses(
        (status = 200, description = "Sign in successfully", body = AuthPayload),
    )
)]
pub async fn sign_in(
    State(state): State<AppState>,
    ValidJson(body): ValidJson<SignInWithPasswordBody>,
) -> Result<Json<AuthPayload>, AppError> {
    let resp = state
        .auth_service
        .sign_in_with_password(SignInWithPasswordInput {
            email: body.email,
            password: body.password,
        })
        .await?;

    Ok(Json(resp.into()))
}

#[utoipa::path(
    get,
    path = "/renew",
    params(
        RenewAccessTokenQuery,
    ),
    responses(
        (status = 200, description = "Renew access token successfully", body = AuthPayload),
    )
)]
pub async fn renew_access_token(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Query(query): Query<RenewAccessTokenQuery>,
) -> Result<Json<AuthPayload>, AppError> {
    let resp = state
        .auth_service
        .renew_access_token(user.id, query.refresh_token)
        .await?;

    Ok(Json(resp.into()))
}

#[derive(OpenApi)]
#[openapi(
    paths(sign_up, sign_in, renew_access_token, sign_in_with_oauth),
    components(
        schemas(
            SignUpWithPasswordBody,
            SignInWithPasswordBody,
            SignInWithOauthBody,
            AuthPayload,
            Provider
        )
    ),
    tags(
        (name = "crate::api::auth", description = "Auth API")
    )
)]
pub struct AuthApiDoc;
