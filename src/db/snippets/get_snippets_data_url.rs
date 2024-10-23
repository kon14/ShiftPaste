use sqlx::PgExecutor;
use std::collections::HashSet;
use uuid::Uuid;

use crate::domain::types::{SnippetData, UrlSnippet};
use crate::prelude::*;

pub struct GetSnippetsDataUrlDbResponseInner {
    pub snippet_id: Uuid,
    pub url: String,
}

pub async fn get_snippets_data_url<'a>(
    db: impl PgExecutor<'a>,
    snippet_ids: &HashSet<Uuid>,
) -> Result<Vec<GetSnippetsDataUrlDbResponseInner>, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve url snippets data!";

    let snippet_ids: Vec<Uuid> = snippet_ids.iter().cloned().collect();
    sqlx::query_as!(
        GetSnippetsDataUrlDbResponseInner,
        r#"
        SELECT
            snippet_id,
            url
        FROM snippets_data_url
        WHERE
            snippet_id = ANY($1)
        "#,
        &snippet_ids,
    )
    .fetch_all(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}

impl From<GetSnippetsDataUrlDbResponseInner> for UrlSnippet {
    fn from(db_res: GetSnippetsDataUrlDbResponseInner) -> Self {
        UrlSnippet { url: db_res.url }
    }
}

impl From<GetSnippetsDataUrlDbResponseInner> for SnippetData {
    fn from(db_res: GetSnippetsDataUrlDbResponseInner) -> Self {
        SnippetData::URL(db_res.into())
    }
}
