use crate::api::exchange_rate::*;
use crate::api::state::AppState;
use axum::routing::get;
use axum::Router;

pub struct ExchangeRateRouter(Router<AppState>);

impl ExchangeRateRouter {
    pub fn new(_state: AppState) -> Self {
        let routes = Router::new().route("/latest", get(get_latest));

        Self(routes)
    }
}

impl From<ExchangeRateRouter> for Router<AppState> {
    fn from(router: ExchangeRateRouter) -> Self {
        router.0
    }
}
