use sqlx::PgExecutor;
use uuid::Uuid;

use crate::domain::types::{SnippetData, TextSnippet};
use crate::prelude::*;

pub struct GetSnippetDataTextDbResponse {
    pub snippet_id: Uuid,
    pub text: String,
}

pub async fn get_snippet_data_text<'a>(
    db: impl PgExecutor<'a>,
    snippet_id: Uuid,
) -> Result<GetSnippetDataTextDbResponse, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve text snippet data!";

    sqlx::query_as!(
        GetSnippetDataTextDbResponse,
        r#"
        SELECT
            snippet_id,
            text
        FROM snippets_data_text
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

impl From<GetSnippetDataTextDbResponse> for TextSnippet {
    fn from(db_res: GetSnippetDataTextDbResponse) -> Self {
        TextSnippet { text: db_res.text }
    }
}

impl From<GetSnippetDataTextDbResponse> for SnippetData {
    fn from(db_res: GetSnippetDataTextDbResponse) -> Self {
        SnippetData::Text(db_res.into())
    }
}
