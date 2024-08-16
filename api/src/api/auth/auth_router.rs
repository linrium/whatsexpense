use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use axum::Router;

use crate::api::auth::auth_controller::*;
use crate::api::state::AppState;
use crate::mw::user_info_mw;

pub struct AuthRouter(Router<AppState>);

impl AuthRouter {
    pub fn new(state: AppState) -> Self {
        let routes = Router::new()
            .route("/oauth/sign-in", post(sign_in_with_oauth))
            .route("/sign-in", post(sign_in))
            .route("/sign-up", post(sign_up))
            .route(
                "/renew",
                get(renew_access_token)
                    .route_layer(from_fn_with_state(state.clone(), user_info_mw)),
            );

        Self(routes)
    }
}

impl From<AuthRouter> for Router<AppState> {
    fn from(router: AuthRouter) -> Self {
        router.0
    }
}
