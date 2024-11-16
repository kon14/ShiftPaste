mod auth_middleware;
mod login;

pub use auth_middleware::*;
pub use login::*;

use crate::common::state::AppState;

pub fn declare_routes() -> axum::Router<AppState> {
    axum::Router::new().route("/auth/login", axum::routing::post(login))
}
