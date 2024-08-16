use crate::api::category::category_controller::*;
use crate::api::state::AppState;
use crate::mw::authorization_mw;
use axum::middleware::from_fn_with_state;
use axum::routing::get;
use axum::Router;

pub struct CategoryRouter(Router<AppState>);

impl CategoryRouter {
    pub fn new(state: AppState) -> Self {
        let routes = Router::new()
            .route("/", get(list_categories))
            .route_layer(from_fn_with_state(state.clone(), authorization_mw));

        Self(routes)
    }
}

impl From<CategoryRouter> for Router<AppState> {
    fn from(router: CategoryRouter) -> Self {
        router.0
    }
}
