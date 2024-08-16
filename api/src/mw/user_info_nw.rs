use std::str::FromStr;

use crate::api::state::AppState;
use crate::api::user::UserError;
use crate::common::errors::AppError;
use crate::services::jwt::JwtError;
use axum::body::Body;
use axum::extract::{Request, State};
use axum::http;
use axum::http::Response;
use axum::middleware::Next;
use bson::oid::ObjectId;

pub async fn user_info_mw(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, AppError> {
    let auth_header = req
        .headers_mut()
        .get(http::header::AUTHORIZATION)
        .ok_or(JwtError::NotFound)?
        .to_str()
        .map_err(|_| JwtError::NotFound)?;

    let mut header = auth_header.split_whitespace();
    let (_, token) = (header.next(), header.next());
    let secret = state.settings.auth.jwt.access_token.secret_key.as_str();
    let token_data = match state.jwt_service.decode(&secret, token.unwrap_or_default()) {
        Ok(data) => data,
        Err(_) => return Err(JwtError::NotFound.into()),
    };

    let user_id = match ObjectId::from_str(token_data.claims.sub.as_str()) {
        Ok(id) => id,
        Err(_) => return Err(JwtError::UnableToDecode.into()),
    };
    let user = match state.user_service.find_by_id(user_id).await {
        Ok(Some(user)) => user,
        _ => return Err(UserError::NotFound.into()),
    };

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
