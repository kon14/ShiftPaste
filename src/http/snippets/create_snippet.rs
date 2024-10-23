use axum::{extract::State, Json};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::common::state::AppState;
use crate::domain as dmn;
use crate::domain::types::{Snippet, SnippetData};
use crate::prelude::*;

#[derive(Deserialize, ToSchema)]
pub struct CreateSnippetHttpParams {
    pub data: SnippetData,
}

/// Creates a new Snippet.
#[utoipa::path(
    post,
    path = "/snippets",
    responses(
        (status = 200, description = "Success", body = Snippet),
        (status = 500, description = "Failure"),
    ),
)]
pub async fn create_snippet(
    State(state): State<AppState>,
    Json(payload): Json<CreateSnippetHttpParams>,
) -> Result<Json<Snippet>, AppError> {
    let AppState { db } = state;

    let snippet = dmn::snippets::create_snippet(&db, payload.into()).await?;
    Ok(Json(snippet))
}

impl From<CreateSnippetHttpParams> for dmn::snippets::CreateSnippetDmnParams {
    fn from(http_args: CreateSnippetHttpParams) -> Self {
        dmn::snippets::CreateSnippetDmnParams {
            data: http_args.data,
        }
    }
}
