use axum::middleware::from_fn_with_state;
use axum::routing::{delete, patch};
use axum::Router;

use crate::api::state::AppState;
use crate::api::transaction::transaction_controller::*;
use crate::mw::authorization_mw;

pub struct TransactionRouter(Router<AppState>);

impl TransactionRouter {
    pub fn new(state: AppState) -> Self {
        let router = Router::new()
            .route("/", patch(update_transactions))
            .route("/", delete(delete_transactions))
            .route_layer(from_fn_with_state(state.clone(), authorization_mw));

        Self(router)
    }
}

impl From<TransactionRouter> for Router<AppState> {
    fn from(router: TransactionRouter) -> Self {
        router.0
    }
}
