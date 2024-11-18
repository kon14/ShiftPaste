mod auth_middleware;
mod generate_tokens;
mod get_tokens;

pub use auth_middleware::*;
pub use generate_tokens::*;
pub use get_tokens::*;

use axum::middleware;

use crate::common::state::AppState;

pub fn declare_routes(state: AppState) -> axum::Router<AppState> {
    let public_router =
        axum::Router::new().route("/auth/tokens", axum::routing::post(generate_tokens));
    let auth_router = axum::Router::new().route("/auth/tokens", axum::routing::get(get_tokens));
    auth_router
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
        .merge(public_router)
}
