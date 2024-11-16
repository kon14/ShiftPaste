mod create_user;
mod delete_user;
mod get_user;
mod get_users;

pub use create_user::*;
pub use delete_user::*;
pub use get_user::*;
pub use get_users::*;

use axum::middleware;

use super::auth::auth_middleware;
use crate::common::state::AppState;

pub fn declare_routes(state: AppState) -> axum::Router<AppState> {
    let public_router = axum::Router::new().route("/users", axum::routing::post(create_user));
    let auth_router = axum::Router::new()
        .route("/users/:userId", axum::routing::delete(delete_user))
        .route("/users/:userId", axum::routing::get(get_user))
        .route("/users", axum::routing::get(get_users));
    auth_router
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
        .merge(public_router)
}
