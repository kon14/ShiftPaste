use chrono::{DateTime, Utc};
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::common::params::PaginationParams;
use crate::db::types::SnippetVariant;
use crate::prelude::*;

pub struct GetSnippetsDbResponseInner {
    pub id: Uuid,
    pub variant: SnippetVariant,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn get_snippets<'a>(
    db: impl PgExecutor<'a>,
    pagination: &PaginationParams,
    archived_filter: Option<bool>,
) -> Result<Vec<GetSnippetsDbResponseInner>, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve snippets!";

    sqlx::query_as!(
        GetSnippetsDbResponseInner,
        r#"
        SELECT
            id,
            variant as "variant!: SnippetVariant",
            archived,
            created_at,
            updated_at
        FROM snippets
        WHERE
            ($3::boolean IS NULL OR archived = $3::boolean)
        ORDER BY id ASC
        OFFSET $1
        LIMIT $2
        "#,
        pagination.skip as i64,
        pagination.limit as i64,
        archived_filter,
    )
    .fetch_all(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}
