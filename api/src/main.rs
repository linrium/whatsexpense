use std::time::Duration;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_scalar::{Scalar, Servable};

use crate::api::router::ApiRouter;
use crate::api::state::AppState;
use crate::common::errors::ErrorResponse;
use crate::settings::Settings;

mod api;
mod common;
pub mod macros;
mod mw;
mod services;
mod settings;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "whatsexpense_api=info,tower_http=trace,axum::rejection=trace,async_graphql::graphql=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    let settings = Settings::new().unwrap();
    let app_state = AppState::init(settings.clone()).await;

    let app = Router::new()
        .nest("/api/v1", ApiRouter::new(app_state.clone()).into())
        .route("/api-docs/openapi.json", get(openapi))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .fallback(handler_404)
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::TRACE)),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_private_network(true)
                .allow_headers(Any),
        )
        .with_state(app_state)
        .route("/healthz", get(healthz));

    let addr = format!("{}:{}", settings.server.host, settings.server.port);
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

#[derive(OpenApi)]
#[openapi(
    servers((url = "http://0.0.0.0:3000"), (url = "https://whatsexpense-api.onrender.com")),
    modifiers(&SecurityAddon),
    nest(
        (path = "/api/v1/assets", api = crate::api::asset::AssetApiDoc),
        (path = "/api/v1/auth", api = crate::api::auth::AuthApiDoc),
        (path = "/api/v1/invoices", api = crate::api::invoice::InvoiceApiDoc),
        (path = "/api/v1/messages", api = crate::api::message::MessageApiDoc),
        (path = "/api/v1/users", api = crate::api::user::UserApiDoc),
        (path = "/api/v1/reports", api = crate::api::report::ReportApiDoc),
        (path = "/api/v1/transactions", api = crate::api::transaction::TransactionApiDoc),
        (path = "/api/v1/categories", api = crate::api::category::CategoryApiDoc),
    ),
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "Authorization",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            )
        }
    }
}

pub async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn healthz() -> &'static str {
    "OK"
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new("nothing here".to_string())),
    )
}
