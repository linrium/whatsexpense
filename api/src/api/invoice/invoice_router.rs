use axum::extract::DefaultBodyLimit;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use axum::Router;

use crate::api::invoice::invoice_controller::*;
use crate::api::state::AppState;
use crate::mw::authorization_mw;

// 1MB
const MAX_FILE_SIZE: usize = 1024 * 1024;

pub struct InvoiceRouter(Router<AppState>);

impl InvoiceRouter {
    pub fn new(state: AppState) -> Self {
        let routes = Router::new()
            .route(
                "/upload",
                post(upload_invoice).layer(DefaultBodyLimit::max(MAX_FILE_SIZE)),
            )
            .route("/:invoice_id/presigned", get(presigned))
            .layer(from_fn_with_state(state.clone(), authorization_mw));

        Self(routes)
    }
}

impl From<InvoiceRouter> for Router<AppState> {
    fn from(router: InvoiceRouter) -> Self {
        router.0
    }
}
