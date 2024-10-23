mod archive_snippet;
mod create_snippet;
mod get_snippet;
mod get_snippets;
mod patch_snippet;

pub use archive_snippet::*;
pub use create_snippet::*;
pub use get_snippet::*;
pub use get_snippets::*;
pub use patch_snippet::*;

use crate::common::state::AppState;

pub fn declare_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/snippets", axum::routing::post(create_snippet))
        .route("/snippets", axum::routing::get(get_snippets))
        .route("/snippets/:snippetId", axum::routing::get(get_snippet))
        .route("/snippets/:snippetId", axum::routing::patch(patch_snippet))
        .route(
            "/snippets/:snippetId",
            axum::routing::delete(archive_snippet),
        )
}
