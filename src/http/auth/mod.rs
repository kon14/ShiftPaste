mod auth_middleware;
mod get_tokens;
mod login;
mod renew_tokens;
mod revoke_tokens;

pub use auth_middleware::*;
pub use get_tokens::*;
pub use login::*;
pub use renew_tokens::{PATH as RENEW_TOKENS_PATH, *};
pub use revoke_tokens::*;

use axum::middleware;

use crate::common::state::AppState;

pub fn declare_routes(state: AppState) -> axum::Router<AppState> {
    let public_router = axum::Router::new().route("/auth/tokens", axum::routing::post(login));
    let auth_router = axum::Router::new()
        .route("/auth/tokens", axum::routing::get(get_tokens))
        .route("/auth/tokens/renew", axum::routing::post(renew_tokens))
        .route(
            "/auth/tokens/:access_token_id",
            axum::routing::delete(revoke_tokens),
        );
    auth_router
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
        .merge(public_router)
}
