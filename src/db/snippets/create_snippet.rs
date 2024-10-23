use chrono::{DateTime, Utc};
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::db::types::SnippetVariant;
use crate::prelude::*;

pub struct CreateSnippetDbResponse {
    pub id: Uuid,
    pub variant: SnippetVariant,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn create_snippet<'a>(
    db: impl PgExecutor<'a>,
    variant: SnippetVariant,
) -> Result<CreateSnippetDbResponse, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to create snippet!";

    sqlx::query_as!(
        CreateSnippetDbResponse,
        r#"
        INSERT INTO snippets (
            variant
        )
        VALUES ($1)
        RETURNING
            id,
            variant as "variant!: SnippetVariant",
            archived,
            created_at,
            updated_at
        "#,
        variant as SnippetVariant,
    )
    .fetch_one(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}
