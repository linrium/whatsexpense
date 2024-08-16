use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use google_oauth::AsyncClient;
use nanoid::nanoid;
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use reqwest::Client;

use crate::api::auth::types::Provider;
use crate::api::auth::*;
use crate::api::identity::{IdentityServiceDyn, InsertIdentityInput};
use crate::api::user::{CreateUserInput, UserError, UserServiceDyn};
use crate::common::errors::AppError;
use crate::object_id;
use crate::services::apple_auth::AppleJwtClient;
use crate::services::jwt::JwtServiceDyn;
use crate::settings::AuthConfig;

#[async_trait]
pub trait AuthServiceExt: Send + Sync {
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AppError>;
    fn hash_password(&self, password: &str) -> Result<String, AppError>;
    async fn renew_access_token(
        &self,
        user_id: String,
        refresh_token: String,
    ) -> Result<AuthResponse, AppError>;
    async fn sign_in_with_password(
        &self,
        input: SignInWithPasswordInput,
    ) -> Result<AuthResponse, AppError>;
    async fn sign_up_with_password(
        &self,
        input: SignUpWithPasswordInput,
    ) -> Result<AuthResponse, AppError>;
    async fn sign_in_with_apple(&self, code: String) -> Result<AuthResponse, AppError>;
    async fn sign_in_with_google(&self, code: String) -> Result<AuthResponse, AppError>;
}

pub type AuthServiceDyn = Arc<dyn AuthServiceExt + Send + Sync>;

#[derive(Clone)]
pub struct AuthService {
    pub config: AuthConfig,
    pub http_client: Client,
    pub identity_service: IdentityServiceDyn,
    pub user_service: UserServiceDyn,
    pub jwt_service: JwtServiceDyn,
    pub redis_client: redis::Client,
}

impl AuthService {
    async fn has_refresh_token(
        &self,
        conn: &mut MultiplexedConnection,
        user_id: String,
        refresh_token: String,
    ) -> Result<bool, AppError> {
        let key = format!("user:{}:refresh_token:{}", user_id, refresh_token);
        let refresh_token: Option<String> = conn
            .get(&key)
            .await
            .map_err(|e| UserError::Unknown(anyhow!(e)))?;

        Ok(refresh_token.is_some())
    }

    async fn generate_refresh_token(
        &self,
        conn: &mut MultiplexedConnection,
        user_id: String,
    ) -> Result<String, AppError> {
        let refresh_token_config = self.config.jwt.refresh_token.clone();
        let refresh_token = nanoid!(20);

        let key = format!("user:{}:refresh_token:{}", user_id, refresh_token);
        let _: () = conn
            .set_ex(&key, 1, refresh_token_config.expires_in_secs)
            .await
            .map_err(|e| UserError::Unknown(anyhow!(e)))?;

        Ok(refresh_token)
    }

    async fn invalidate_refresh_token(
        &self,
        conn: &mut MultiplexedConnection,
        user_id: String,
        refresh_token: String,
    ) -> Result<(), AppError> {
        let key = format!("user:{}:refresh_token:{}", user_id, refresh_token);
        let _: () = conn
            .del(&key)
            .await
            .map_err(|e| UserError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    fn generate_access_token(&self, user_id: String) -> Result<String, AppError> {
        let access_token_config = self.config.jwt.access_token.clone();
        let access_token = self.jwt_service.encode(
            &access_token_config.secret_key,
            access_token_config.expires_in_secs as i64,
            user_id,
        )?;

        Ok(access_token)
    }

    async fn sign_in_with(
        &self,
        profile: UserProfile,
        identity_data: serde_json::Value,
    ) -> Result<AuthResponse, AppError> {
        let user = self
            .user_service
            .find_by_email(profile.email.clone())
            .await?;
        if let Some(user) = user {
            let mut conn = self
                .redis_client
                .get_multiplexed_async_connection()
                .await
                .map_err(|e| UserError::Unknown(anyhow!(e)))?;

            let access_token = self.generate_access_token(user.id.clone())?;
            let refresh_token = self
                .generate_refresh_token(&mut conn, user.id.clone())
                .await?;

            return Ok(AuthResponse {
                access_token,
                refresh_token,
                user: user.clone(),
            });
        }

        let username = profile.generate_username();
        let response = self
            .sign_up_with_password(SignUpWithPasswordInput {
                email: profile.email.clone(),
                password: nanoid!(10),
                picture: profile.picture.clone(),
                username: Some(username),
                given_name: profile.given_name.clone(),
                family_name: profile.family_name.clone(),
            })
            .await?;

        let user = response.user.clone();
        self.identity_service
            .insert_one(InsertIdentityInput {
                email: Some(profile.email.clone()),
                user_id: object_id!(&response.user.id),
                identity_data,
                name: user.full_name,
                provider: Provider::Google,
                sub: profile.sub.clone(),
            })
            .await
            .map_err(|e| AuthError::Unknown(e.into()))?;

        Ok(response.clone())
    }
}

#[async_trait]
impl AuthServiceExt for AuthService {
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        verify(password, hash).map_err(|_| AuthError::InvalidPassword.into())
    }

    fn hash_password(&self, password: &str) -> Result<String, AppError> {
        hash(password, DEFAULT_COST).map_err(|_| AuthError::Hashing.into())
    }

    async fn renew_access_token(
        &self,
        user_id: String,
        refresh_token: String,
    ) -> Result<AuthResponse, AppError> {
        let mut conn = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| UserError::Unknown(anyhow!(e)))?;

        let has_refresh_token = self
            .has_refresh_token(&mut conn, user_id.clone(), refresh_token.clone())
            .await?;

        if !has_refresh_token {
            return Err(AuthError::RefreshTokenExpired.into());
        }

        let user = self
            .user_service
            .find_by_id(object_id!(&user_id))
            .await?
            .ok_or(UserError::NotFound)?;
        let new_access_token = self.generate_access_token(user_id.clone())?;
        let new_refresh_token = self
            .generate_refresh_token(&mut conn, user_id.clone())
            .await?;
        self.invalidate_refresh_token(&mut conn, user_id.clone(), refresh_token)
            .await?;

        Ok(AuthResponse {
            access_token: new_access_token,
            refresh_token: new_refresh_token,
            user,
        })
    }

    async fn sign_in_with_password(
        &self,
        input: SignInWithPasswordInput,
    ) -> Result<AuthResponse, AppError> {
        let user = self
            .user_service
            .find_by_email(input.email)
            .await?
            .ok_or(UserError::NotFound)?;

        if !self.verify_password(&input.password, &user.encrypted_password)? {
            return Err(AuthError::WrongCredentials.into());
        }

        let mut conn = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| UserError::Unknown(anyhow!(e)))?;
        let access_token = self.generate_access_token(user.id.clone())?;
        let refresh_token = self
            .generate_refresh_token(&mut conn, user.id.clone())
            .await?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            user,
        })
    }

    async fn sign_up_with_password(
        &self,
        input: SignUpWithPasswordInput,
    ) -> Result<AuthResponse, AppError> {
        let user = self
            .user_service
            .insert_one(CreateUserInput {
                email: input.email.clone(),
                full_name: input.email.clone(),
                username: input.username.unwrap_or(nanoid!(10)),
                regions: vec!["us".to_string(), "jp".to_string(), "vn".to_string()],
                currency: "USD".to_string(),
                language: "en".to_string(),
                family_name: input.family_name.unwrap_or_default(),
                given_name: input.given_name.unwrap_or_default(),
                picture: input.picture.unwrap_or_default(),
                encrypted_password: self.hash_password(&input.password)?,
            })
            .await?;

        let mut conn = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| UserError::Unknown(anyhow!(e)))?;
        let access_token = self.generate_access_token(user.id.clone())?;
        let refresh_token = self
            .generate_refresh_token(&mut conn, user.id.clone())
            .await?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            user,
        })
    }

    async fn sign_in_with_apple(&self, code: String) -> Result<AuthResponse, AppError> {
        let app_bundle_ids = self
            .config
            .apple
            .app_bundle_ids
            .split(',')
            .collect::<Vec<_>>();
        let mut client = AppleJwtClient::new(&app_bundle_ids);
        let payload = client
            .decode(code.as_str())
            .await
            .map_err(|e| AuthError::Unknown(e.into()))?;

        let now = chrono::Utc::now().timestamp() as u64;
        if payload.expiration_time.unwrap_or_default() > now {
            return Err(AuthError::TokenExpired.into());
        }

        let Some(email) = &payload.email else {
            return Err(AuthError::WrongCredentials.into());
        };

        let identity_data =
            serde_json::to_value(&payload.clone()).map_err(|e| AuthError::Unknown(e.into()))?;

        self.sign_in_with(
            UserProfile {
                email: email.clone(),
                picture: None,
                given_name: None,
                family_name: None,
                sub: payload.user_id,
            },
            identity_data,
        )
        .await
    }

    async fn sign_in_with_google(&self, code: String) -> Result<AuthResponse, AppError> {
        let client_id = self.config.google.client_id.clone();
        let client = AsyncClient::new(client_id);
        let payload = client
            .validate_id_token(code)
            .await
            .map_err(|e| AuthError::Unknown(e.into()))?;

        let now = chrono::Utc::now().timestamp() as u64;
        if payload.exp > now {
            return Err(AuthError::TokenExpired.into());
        }

        let Some(email) = payload.email.clone() else {
            return Err(AuthError::WrongCredentials.into());
        };

        let identity_data =
            serde_json::to_value(&payload).map_err(|e| AuthError::Unknown(e.into()))?;

        self.sign_in_with(
            UserProfile {
                email: email.clone(),
                picture: payload.picture.clone(),
                given_name: payload.given_name.clone(),
                family_name: payload.family_name.clone(),
                sub: payload.sub.clone(),
            },
            identity_data,
        )
        .await
    }
}
