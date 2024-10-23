mod generate_qr;

pub use generate_qr::*;

use crate::common::state::AppState;

pub fn declare_routes() -> axum::Router<AppState> {
    axum::Router::new().route("/qr/:snippetId", axum::routing::post(generate_qr))
}
