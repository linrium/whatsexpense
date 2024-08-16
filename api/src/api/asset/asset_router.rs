use crate::api::asset::asset_controller::{list_currencies, list_languages, list_regions};
use crate::api::state::AppState;
use crate::mw::authorization_mw;
use axum::middleware::from_fn_with_state;
use axum::routing::get;
use axum::Router;

pub struct AssetRouter(Router<AppState>);

impl AssetRouter {
    pub fn new(state: AppState) -> Self {
        let router = Router::new()
            .route("/currencies", get(list_currencies))
            .route("/regions", get(list_regions))
            .route("/languages", get(list_languages))
            .route_layer(from_fn_with_state(state.clone(), authorization_mw));

        Self(router)
    }
}

impl From<AssetRouter> for Router<AppState> {
    fn from(router: AssetRouter) -> Self {
        router.0
    }
}
