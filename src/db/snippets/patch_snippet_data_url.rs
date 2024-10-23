use sqlx::PgExecutor;
use uuid::Uuid;

use crate::domain::types::{SnippetData, UrlSnippet};
use crate::prelude::*;

pub struct PatchSnippetDataUrlDbResponse {
    pub snippet_id: Uuid,
    pub url: String,
}

pub async fn patch_snippet_data_url<'a>(
    db: impl PgExecutor<'a>,
    snippet_id: &Uuid,
    data: UrlSnippet,
) -> Result<PatchSnippetDataUrlDbResponse, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve url snippet data!";

    sqlx::query_as!(
        PatchSnippetDataUrlDbResponse,
        r#"
        UPDATE snippets_data_url
        SET
            url = $2
        WHERE snippet_id = $1
        RETURNING
            snippet_id,
            url
        "#,
        snippet_id,
        data.url,
    )
    .fetch_one(db)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => {
            AppError::not_found(format!("Snippet ({snippet_id}) doesn't exist!"))
        }
        _ => AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()),
    })
}

impl From<PatchSnippetDataUrlDbResponse> for UrlSnippet {
    fn from(db_res: PatchSnippetDataUrlDbResponse) -> Self {
        UrlSnippet { url: db_res.url }
    }
}

impl From<PatchSnippetDataUrlDbResponse> for SnippetData {
    fn from(db_res: PatchSnippetDataUrlDbResponse) -> Self {
        SnippetData::URL(db_res.into())
    }
}
