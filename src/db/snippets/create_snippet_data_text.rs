use sqlx::PgExecutor;
use uuid::Uuid;

use crate::domain::types::{SnippetData, TextSnippet};
use crate::prelude::*;

pub struct CreateSnippetDataTextDbResponse {
    pub snippet_id: Uuid,
    pub text: String,
}

pub async fn create_snippet_data_text<'a>(
    db: impl PgExecutor<'a>,
    snippet_id: &Uuid,
    data: TextSnippet,
) -> Result<CreateSnippetDataTextDbResponse, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to create snippet_data_text!";

    sqlx::query_as!(
        CreateSnippetDataTextDbResponse,
        r#"
        INSERT INTO snippets_data_text (
            snippet_id,
            text
        )
        VALUES ($1, $2)
        RETURNING
            snippet_id,
            text
        "#,
        snippet_id,
        data.text,
    )
    .fetch_one(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}

impl From<CreateSnippetDataTextDbResponse> for TextSnippet {
    fn from(db_res: CreateSnippetDataTextDbResponse) -> Self {
        TextSnippet { text: db_res.text }
    }
}

impl From<CreateSnippetDataTextDbResponse> for SnippetData {
    fn from(db_res: CreateSnippetDataTextDbResponse) -> Self {
        SnippetData::Text(db_res.into())
    }
}
