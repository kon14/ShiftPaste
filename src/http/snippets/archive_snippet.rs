use axum::extract::{Path, State};
use uuid::Uuid;

use crate::common::state::AppState;
use crate::domain as dmn;
use crate::prelude::*;

/// Archives a Snippet.
#[utoipa::path(
    delete,
    path = "/snippets/{snippet_id}",
    params(
        ("snippet_id" = Uuid, Path),
    ),
    responses(
        (status = 200, description = "Success", body = String),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Failure"),
    ),
)]
pub async fn archive_snippet(
    State(state): State<AppState>,
    Path(snippet_id): Path<Uuid>,
) -> Result<String, AppError> {
    let AppState { db } = state;

    dmn::snippets::archive_snippet(&db, &snippet_id).await?;
    Ok(format!("Snippet ({}) archived successfully.", snippet_id))
}
