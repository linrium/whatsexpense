use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, post};
use axum::Router;

use crate::api::message::message_controller::*;
use crate::api::state::AppState;
use crate::mw::authorization_mw;

pub struct MessageRouter(Router<AppState>);

impl MessageRouter {
    pub fn new(state: AppState) -> Self {
        let routes = Router::new()
            .route("/", get(list_messages))
            .route("/", post(create_message))
            .route("/:id", delete(delete_message))
            .route("/:id/transactions", get(list_transactions))
            .route_layer(from_fn_with_state(state.clone(), authorization_mw));

        Self(routes)
    }
}

impl From<MessageRouter> for Router<AppState> {
    fn from(router: MessageRouter) -> Self {
        router.0
    }
}
