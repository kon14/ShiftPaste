use sqlx::PgExecutor;
use std::collections::HashSet;
use uuid::Uuid;

use crate::domain::types::{SnippetData, TextSnippet};
use crate::prelude::*;

pub struct GetSnippetsDataTextDbResponseInner {
    pub snippet_id: Uuid,
    pub text: String,
}

pub async fn get_snippets_data_text<'a>(
    db: impl PgExecutor<'a>,
    snippet_ids: &HashSet<Uuid>,
) -> Result<Vec<GetSnippetsDataTextDbResponseInner>, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve text snippets data!";

    let snippet_ids: Vec<Uuid> = snippet_ids.iter().cloned().collect();
    sqlx::query_as!(
        GetSnippetsDataTextDbResponseInner,
        r#"
        SELECT
            snippet_id,
            text
        FROM snippets_data_text
        WHERE
            snippet_id = ANY($1)
        "#,
        &snippet_ids,
    )
    .fetch_all(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}

impl From<GetSnippetsDataTextDbResponseInner> for TextSnippet {
    fn from(db_res: GetSnippetsDataTextDbResponseInner) -> Self {
        TextSnippet { text: db_res.text }
    }
}

impl From<GetSnippetsDataTextDbResponseInner> for SnippetData {
    fn from(db_res: GetSnippetsDataTextDbResponseInner) -> Self {
        SnippetData::Text(db_res.into())
    }
}
