use sqlx::PgExecutor;
use uuid::Uuid;

use crate::domain::types::{SnippetData, TextSnippet};
use crate::prelude::*;

pub struct PatchSnippetDataTextDbResponse {
    pub text: String,
}

pub async fn patch_snippet_data_text<'a>(
    db: impl PgExecutor<'a>,
    snippet_id: Uuid,
    data: TextSnippet,
) -> Result<PatchSnippetDataTextDbResponse, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to patch text snippet data!";

    sqlx::query_as!(
        PatchSnippetDataTextDbResponse,
        r#"
        UPDATE snippets_data_text
        SET
            text = $2
        WHERE snippet_id = $1
        RETURNING
            text
        "#,
        snippet_id,
        data.text,
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

impl From<PatchSnippetDataTextDbResponse> for TextSnippet {
    fn from(db_res: PatchSnippetDataTextDbResponse) -> Self {
        TextSnippet { text: db_res.text }
    }
}

impl From<PatchSnippetDataTextDbResponse> for SnippetData {
    fn from(db_res: PatchSnippetDataTextDbResponse) -> Self {
        SnippetData::Text(db_res.into())
    }
}
