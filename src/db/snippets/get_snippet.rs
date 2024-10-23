use chrono::{DateTime, Utc};
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::db::types::SnippetVariant;
use crate::prelude::*;

pub struct GetSnippetDbResponse {
    pub id: Uuid,
    pub variant: SnippetVariant,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn get_snippet<'a>(
    db: impl PgExecutor<'a>,
    snippet_id: &Uuid,
    archived_filter: Option<bool>,
) -> Result<GetSnippetDbResponse, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve snippet!";

    sqlx::query_as!(
        GetSnippetDbResponse,
        r#"
        SELECT
            id,
            variant as "variant!: SnippetVariant",
            archived,
            created_at,
            updated_at
        FROM snippets
        WHERE
            id = $1
            AND ($2::boolean IS NULL OR archived = $2::boolean)
        "#,
        snippet_id,
        archived_filter,
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
