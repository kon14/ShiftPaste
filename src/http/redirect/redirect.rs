use axum::extract::State;
use axum::{extract::Path, response::Redirect};
use uuid::Uuid;

use crate::common::state::AppState;
use crate::common::utils::get_app_redirect_url;
use crate::domain as dmn;
use crate::domain::types::SnippetData;
use crate::prelude::*;

/// Redirects to the Snippet URL or data retrieval page. (Warning: SwaggerUI automatically follows redirects. Use curl instead.)
#[utoipa::path(
    get,
    path = "/redirect/{snippet_id}",
    params(
        ("snippet_id" = Uuid, Path),
    ),
    responses(
        (
            status = 302,
            description = "Redirection",
            headers(
                ("Location" = String, description = "URL pointing to the referenced Snippet data.")
            ),
        ),
        (status = 404, description = "Not Found"),
    ),
)]
pub async fn redirect(
    State(state): State<AppState>,
    Path(snippet_id): Path<Uuid>,
) -> Result<Redirect, AppError> {
    let AppState { db } = state;

    let snippet = dmn::snippets::get_snippet(&db, snippet_id).await?;

    let redirect_url = match snippet.data {
        SnippetData::URL(data) => data.url,
        _ => format!("{}/{}", get_app_redirect_url(), snippet.id),
    };
    Ok(Redirect::to(&redirect_url))
}
