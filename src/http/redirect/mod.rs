mod redirect;

pub use redirect::*;

use crate::common::state::AppState;

pub fn declare_routes() -> axum::Router<AppState> {
    axum::Router::new().route("/redirect/:snippetId", axum::routing::get(redirect))
}
