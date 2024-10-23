use sqlx::PgExecutor;
use uuid::Uuid;

use crate::prelude::*;

pub async fn archive_snippet<'a>(
    db: impl PgExecutor<'a>,
    snippet_id: &Uuid,
) -> Result<(), AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to archive snippet!";

    sqlx::query!(
        r#"
        UPDATE snippets
        SET
            archived = true
        WHERE id = $1 AND archived = false
        "#,
        snippet_id,
    )
    .execute(db)
    .await
    .map(|_| ())
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => {
            AppError::not_found(format!("Snippet ({snippet_id}) doesn't exist!"))
        }
        _ => AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()),
    })
}
