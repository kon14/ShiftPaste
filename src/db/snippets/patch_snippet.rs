use chrono::{DateTime, Utc};
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::db::types::SnippetVariant;
use crate::prelude::*;

pub struct PatchSnippetDbResponse {
    pub id: Uuid,
    pub variant: SnippetVariant,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn patch_snippet<'a>(
    db: impl PgExecutor<'a>,
    snippet_id: &Uuid,
    variant: SnippetVariant,
) -> Result<PatchSnippetDbResponse, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to patch snippet!";

    sqlx::query_as!(
        PatchSnippetDbResponse,
        r#"
        UPDATE snippets
        SET
            variant = $2
        WHERE id = $1
        RETURNING
            id,
            variant as "variant!: SnippetVariant",
            archived,
            created_at,
            updated_at
        "#,
        snippet_id,
        variant as SnippetVariant,
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
