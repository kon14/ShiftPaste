use axum::extract::Path;
use axum::{extract::State, Json};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::common::state::AppState;
use crate::domain as dmn;
use crate::domain::types::{Snippet, SnippetData};
use crate::prelude::*;

#[derive(Deserialize, ToSchema)]
pub struct PatchSnippetHttpParams {
    pub data: SnippetData,
}

/// Patches a Snippet.
#[utoipa::path(
    patch,
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
pub async fn patch_snippet(
    State(state): State<AppState>,
    Path(snippet_id): Path<Uuid>,
    Json(payload): Json<PatchSnippetHttpParams>,
) -> Result<Json<Snippet>, AppError> {
    let AppState { db } = state;

    let snippet = dmn::snippets::patch_snippet(&db, snippet_id, payload.into()).await?;
    Ok(Json(snippet))
}

impl From<PatchSnippetHttpParams> for dmn::snippets::PatchSnippetDmnParams {
    fn from(http_args: PatchSnippetHttpParams) -> Self {
        dmn::snippets::PatchSnippetDmnParams {
            data: http_args.data,
        }
    }
}
