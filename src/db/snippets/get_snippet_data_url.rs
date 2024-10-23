use sqlx::PgExecutor;
use uuid::Uuid;

use crate::domain::types::{SnippetData, UrlSnippet};
use crate::prelude::*;

pub struct GetSnippetDataUrlDbResponse {
    pub snippet_id: Uuid,
    pub url: String,
}

pub async fn get_snippet_data_url<'a>(
    db: impl PgExecutor<'a>,
    snippet_id: &Uuid,
) -> Result<GetSnippetDataUrlDbResponse, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve url snippet data!";

    sqlx::query_as!(
        GetSnippetDataUrlDbResponse,
        r#"
        SELECT
            snippet_id,
            url
        FROM snippets_data_url
        WHERE snippet_id = $1
        "#,
        snippet_id,
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

impl From<GetSnippetDataUrlDbResponse> for UrlSnippet {
    fn from(db_res: GetSnippetDataUrlDbResponse) -> Self {
        UrlSnippet { url: db_res.url }
    }
}

impl From<GetSnippetDataUrlDbResponse> for SnippetData {
    fn from(db_res: GetSnippetDataUrlDbResponse) -> Self {
        SnippetData::URL(db_res.into())
    }
}
