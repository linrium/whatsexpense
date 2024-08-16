use axum::Router;

use crate::api::asset::AssetRouter;
use crate::api::auth::AuthRouter;
use crate::api::category::CategoryRouter;
use crate::api::exchange_rate::ExchangeRateRouter;
use crate::api::invoice::InvoiceRouter;
use crate::api::message::MessageRouter;
use crate::api::report::ReportRouter;
use crate::api::state::AppState;
use crate::api::transaction::TransactionRouter;
use crate::api::user::UserRouter;

pub struct ApiRouter(Router<AppState>);

impl ApiRouter {
    pub fn new(state: AppState) -> Self {
        let routes = Router::new()
            .nest("/auth", AuthRouter::new(state.clone()).into())
            .nest("/assets", AssetRouter::new(state.clone()).into())
            .nest("/categories", CategoryRouter::new(state.clone()).into())
            .nest("/users", UserRouter::new(state.clone()).into())
            .nest(
                "/exchange-rates",
                ExchangeRateRouter::new(state.clone()).into(),
            )
            .nest("/messages", MessageRouter::new(state.clone()).into())
            .nest("/invoices", InvoiceRouter::new(state.clone()).into())
            .nest("/reports", ReportRouter::new(state.clone()).into())
            .nest("/transactions", TransactionRouter::new(state).into());

        Self(routes)
    }
}

impl From<ApiRouter> for Router<AppState> {
    fn from(router: ApiRouter) -> Self {
        router.0
    }
}
