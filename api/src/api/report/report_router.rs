use axum::middleware::from_fn_with_state;
use axum::routing::get;
use axum::Router;

use crate::api::report::report_controller::*;
use crate::api::state::AppState;
use crate::mw::authorization_mw;

pub struct ReportRouter(Router<AppState>);

impl ReportRouter {
    pub fn new(state: AppState) -> Self {
        let router = Router::new()
            .route("/range", get(report_expenses_by_range))
            .route_layer(from_fn_with_state(state.clone(), authorization_mw));

        Self(router)
    }
}

impl From<ReportRouter> for Router<AppState> {
    fn from(router: ReportRouter) -> Self {
        router.0
    }
}
