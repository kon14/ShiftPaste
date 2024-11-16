use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::common::state::AppState;
use crate::domain as dmn;
use crate::domain::types::Snippet;
use crate::prelude::*;

/// Retrieves a Snippet.
#[utoipa::path(
    get,
    path = "/snippets/{snippet_id}",
    params(
        ("snippet_id" = Uuid, Path),
    ),
    responses(
        (status = 200, description = "Success", body = Snippet),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Failure"),
    ),
)]
pub async fn get_snippet(
    State(state): State<AppState>,
    Path(snippet_id): Path<Uuid>,
) -> Result<Json<Snippet>, AppError> {
    let AppState { db } = state;

    let snippet = dmn::snippets::get_snippet(&db, snippet_id).await?;
    Ok(Json(snippet))
}
