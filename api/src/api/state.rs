use std::sync::Arc;

use async_openai::config::OpenAIConfig;
use async_openai::Client as OpenAIClient;
use mongodb::Client;

use crate::api::auth::{AuthService, AuthServiceDyn};
use crate::api::category::{CategoryService, CategoryServiceDyn};
use crate::api::exchange_rate::{ExchangeRateRepo, ExchangeRateService, ExchangeRateServiceDyn};
use crate::api::identity::{IdentityRepo, IdentityService, IdentityServiceDyn};
use crate::api::infer::{
    InferServiceFactory, InferServiceFactoryDyn, InvoiceInferService, TextInferService,
};
use crate::api::invoice::{InvoiceRepo, InvoiceService, InvoiceServiceDyn};
use crate::api::message::{MessageRepo, MessageService, MessageServiceDyn};
use crate::api::report::{ReportRepo, ReportService, ReportServiceDyn};
use crate::api::transaction::{TransactionRepo, TransactionService, TransactionServiceDyn};
use crate::api::user::{UserRepo, UserService, UserServiceDyn};
use crate::services::currencyapi::CurrencyApiService;
use crate::services::gcp::auth::GCPAuthService;
use crate::services::gcp::vision::VisionService;
use crate::services::jwt::{JwtService, JwtServiceDyn};
use crate::services::llm::{AnthropicService, OpenAIService};
use crate::services::r2::{R2Service, R2ServiceDyn};
use crate::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub http_client: reqwest::Client,
    pub redis_client: redis::Client,
    pub user_service: UserServiceDyn,
    pub identity_service: IdentityServiceDyn,
    pub auth_service: AuthServiceDyn,
    pub jwt_service: JwtServiceDyn,
    pub exchange_rate_service: ExchangeRateServiceDyn,
    pub transaction_service: TransactionServiceDyn,
    pub message_service: MessageServiceDyn,
    pub invoice_service: InvoiceServiceDyn,
    pub category_service: CategoryServiceDyn,
    pub r2_service: R2ServiceDyn,
    pub infer_service_factory: InferServiceFactoryDyn,
    pub report_service: ReportServiceDyn,
}

impl AppState {
    pub async fn init(settings: Settings) -> Self {
        let mongo_client = Client::with_uri_str(settings.database.url.as_str())
            .await
            .unwrap();
        let database = mongo_client.database(settings.database.name.as_str());

        let redis_client = redis::Client::open(settings.redis.url.as_str()).unwrap();

        // settings
        let settings = Arc::new(settings);

        // reqwest
        let http_client = reqwest::Client::new();

        // llm
        let openai_client = OpenAIClient::with_config(
            OpenAIConfig::default()
                .with_api_base(settings.llm.openai.base_url.as_str())
                .with_api_key(settings.llm.openai.api_key.as_str()),
        )
        .with_http_client(http_client.clone());

        let openai_service = Arc::new(OpenAIService {
            client: openai_client,
        });

        let anthropic_service = Arc::new(AnthropicService {
            http_client: http_client.clone(),
            api_key: settings.llm.anthropic.api_key.clone(),
        });

        // infer
        let text_infer_service = Arc::new(TextInferService {
            llm_service: openai_service.clone(),
        });
        let invoice_infer_service = Arc::new(InvoiceInferService {
            llm_service: anthropic_service.clone(),
        });
        let infer_service_factory = Arc::new(InferServiceFactory {
            text_infer_service: text_infer_service.clone(),
            invoice_infer_service: invoice_infer_service.clone(),
        });

        // r2
        let r2_service = Arc::new(R2Service::new(settings.r2.clone()));

        // gcp
        let gcp_auth_service = Arc::new(GCPAuthService::new(
            settings.gcp.clone(),
            redis_client.clone(),
        ));
        let gcp_vision_service = Arc::new(VisionService {
            http_client: http_client.clone(),
            gcp_auth_service: gcp_auth_service.clone(),
        });

        // identity
        let identity_repo = Arc::new(IdentityRepo {
            collection: database.collection("identities"),
        });
        let identity_service = Arc::new(IdentityService {
            repo: identity_repo,
        });

        // user
        let user_repo = Arc::new(UserRepo {
            collection: database.collection("users"),
        });
        let user_service = Arc::new(UserService {
            repo: user_repo,
            redis_client: redis_client.clone(),
        });

        // auth
        let jwt_service = Arc::new(JwtService {
            config: settings.auth.jwt.clone(),
        });
        let auth_service = Arc::new(AuthService {
            config: settings.auth.clone(),
            identity_service: identity_service.clone(),
            user_service: user_service.clone(),
            jwt_service: jwt_service.clone(),
            http_client: http_client.clone(),
            redis_client: redis_client.clone(),
        });

        // currency api
        let currencyapi_service = Arc::new(CurrencyApiService {
            settings: settings.clone(),
            http_client: http_client.clone(),
        });

        // exchange rate
        let exchange_rate_repo = Arc::new(ExchangeRateRepo {
            collection: database.collection("exchange_rates"),
        });
        let exchange_rate_service = Arc::new(ExchangeRateService {
            currencyapi_service: currencyapi_service.clone(),
            repo: exchange_rate_repo,
        });

        // category
        let category_service = Arc::new(CategoryService {});

        // transaction
        let transaction_repo = Arc::new(TransactionRepo {
            collection: database.collection("transactions"),
        });
        let transaction_service = Arc::new(TransactionService {
            repo: transaction_repo,
        });

        // invoice
        let invoice_repo = Arc::new(InvoiceRepo {
            collection: database.collection("invoices"),
        });
        let invoice_service = Arc::new(InvoiceService {
            repo: invoice_repo,
            r2_service: r2_service.clone(),
            gcp_vision_service: gcp_vision_service.clone(),
            config: settings.invoice.clone(),
        });

        // message
        let message_repo = Arc::new(MessageRepo {
            collection: database.collection("messages"),
        });
        let message_service = Arc::new(MessageService {
            repo: message_repo,
            mongo_client: mongo_client.clone(),
            transaction_service: transaction_service.clone(),
            invoice_service: invoice_service.clone(),
        });

        // report
        let report_repo = Arc::new(ReportRepo {
            transaction_col: database.collection("transactions"),
        });
        let report_service = Arc::new(ReportService { repo: report_repo });

        Self {
            settings,
            http_client,
            redis_client,
            user_service,
            identity_service,
            auth_service,
            jwt_service,
            exchange_rate_service,
            transaction_service,
            message_service,
            invoice_service,
            category_service,
            r2_service,
            infer_service_factory,
            report_service,
        }
    }
}
