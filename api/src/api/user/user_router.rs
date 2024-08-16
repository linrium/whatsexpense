use crate::api::state::AppState;
use crate::api::user::user_controller::*;
use crate::mw::authorization_mw;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, patch};
use axum::Router;

pub struct UserRouter(Router<AppState>);

impl UserRouter {
    pub fn new(state: AppState) -> Self {
        let routes = Router::new()
            .route("/me", get(get_user))
            .route("/me", patch(update_user))
            .route("/me", delete(delete_user))
            .route_layer(from_fn_with_state(state.clone(), authorization_mw));

        Self(routes)
    }
}

impl From<UserRouter> for Router<AppState> {
    fn from(router: UserRouter) -> Self {
        router.0
    }
}
