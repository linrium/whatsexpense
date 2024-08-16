use std::sync::Arc;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};

use crate::api::auth::{Claims, Role};
use crate::services::jwt::JwtError;
use crate::settings::JwtAuthConfig;

pub trait JwtServiceExt: Send + Sync {
    fn encode(&self, secret: &str, expires_in_secs: i64, sub: String) -> Result<String, JwtError>;
    fn decode(&self, secret: &str, token: &str) -> Result<TokenData<Claims>, JwtError>;
}

pub type JwtServiceDyn = Arc<dyn JwtServiceExt + Send + Sync>;

pub struct JwtService {
    pub config: JwtAuthConfig,
}

impl JwtServiceExt for JwtService {
    fn encode(&self, secret: &str, expires_in_secs: i64, sub: String) -> Result<String, JwtError> {
        let iss = self.config.issuer.clone();

        let now = Utc::now();
        let expires_at = Duration::seconds(expires_in_secs);
        let exp = (now + expires_at).timestamp();
        let iat = now.timestamp();
        let claims = Claims {
            iss,
            sub,
            iat,
            exp,
            role: Role::User,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|_| JwtError::UnableToEncode)
    }

    fn decode(&self, secret: &str, token: &str) -> Result<TokenData<Claims>, JwtError> {
        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| JwtError::UnableToDecode)
    }
}
