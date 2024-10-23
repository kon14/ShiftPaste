use crate::domain::types::{SnippetData, UrlSnippet};
use crate::prelude::*;

use sqlx::PgExecutor;
use uuid::Uuid;

pub struct CreateSnippetDataUrlDbResponse {
    pub snippet_id: Uuid,
    pub url: String,
}

pub async fn create_snippet_data_url<'a>(
    db: impl PgExecutor<'a>,
    snippet_id: &Uuid,
    data: UrlSnippet,
) -> Result<CreateSnippetDataUrlDbResponse, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to create snippet_data_url!";

    sqlx::query_as!(
        CreateSnippetDataUrlDbResponse,
        r#"
        INSERT INTO snippets_data_url (
            snippet_id,
            url
        )
        VALUES ($1, $2)
        RETURNING
            snippet_id,
            url
        "#,
        snippet_id,
        data.url,
    )
    .fetch_one(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}

impl From<CreateSnippetDataUrlDbResponse> for UrlSnippet {
    fn from(db_res: CreateSnippetDataUrlDbResponse) -> Self {
        UrlSnippet { url: db_res.url }
    }
}

impl From<CreateSnippetDataUrlDbResponse> for SnippetData {
    fn from(db_res: CreateSnippetDataUrlDbResponse) -> Self {
        SnippetData::URL(db_res.into())
    }
}
