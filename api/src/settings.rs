use config::{Config, ConfigError, Environment, File};
use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub addr: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct DatabaseConfig {
    pub url: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct GoogleAuthConfig {
    pub client_id: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct AppleAuthConfig {
    pub app_bundle_ids: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct JwtTokenConfig {
    pub secret_key: String,
    pub expires_in_secs: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct JwtAuthConfig {
    pub issuer: String,
    pub access_token: JwtTokenConfig,
    pub refresh_token: JwtTokenConfig,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct AuthConfig {
    pub jwt: JwtAuthConfig,
    pub google: GoogleAuthConfig,
    pub apple: AppleAuthConfig,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct CurrencyapiConfig {
    pub url: String,
    pub api_key: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct OpenAIConfig {
    pub base_url: String,
    pub api_key: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(unused)]
pub struct AnthropicConfig {
    pub api_key: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct LLMConfig {
    pub openai: OpenAIConfig,
    pub anthropic: AnthropicConfig,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(unused)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(unused)]
pub struct R2Config {
    pub account_id: String,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(unused)]
pub struct GCPConfig {
    pub service_account: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(unused)]
pub struct InvoiceConfig {
    pub bucket: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub currencyapi: CurrencyapiConfig,
    pub llm: LLMConfig,
    pub redis: RedisConfig,
    pub r2: R2Config,
    pub gcp: GCPConfig,
    pub invoice: InvoiceConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();

        let rust_env = env::var("RUST_ENV").unwrap_or("default".to_string());
        let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
        let port = env::var("PORT").unwrap_or("3000".to_string());

        let s = Config::builder()
            .add_source(File::with_name(&format!("config/{}", rust_env)).required(false))
            .add_source(
                Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("__"),
            )
            .set_override("server.port", port)?
            .set_override("server.host", host)?
            .build()?;

        s.try_deserialize()
    }
}
