use axum::{
    extract::{Query, State},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

use crate::common::params::PaginationParams;
use crate::common::state::AppState;
use crate::domain as dmn;
use crate::domain::types::Snippet;
use crate::prelude::*;

#[derive(Serialize, ToSchema)]
pub struct GetSnippetsHttpResponse {
    pub snippets: Vec<Snippet>,
    pub count: u32,
}

/// Retrieves multiple Snippets.
#[utoipa::path(
    get,
    path = "/snippets",
    params(
        PaginationParams,
    ),
    responses(
        (status = 200, description = "Success", body = GetSnippetsHttpResponse),
        (status = 500, description = "Failure"),
    ),
)]
pub async fn get_snippets(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<GetSnippetsHttpResponse>, AppError> {
    let AppState { db } = state;

    let snippets = dmn::snippets::get_snippets(&db, &pagination).await?;
    Ok(Json(snippets.into()))
}

impl From<dmn::snippets::GetSnippetsDmnResponse> for GetSnippetsHttpResponse {
    fn from(dmn_res: dmn::snippets::GetSnippetsDmnResponse) -> Self {
        GetSnippetsHttpResponse {
            snippets: dmn_res.snippets,
            count: dmn_res.count,
        }
    }
}
