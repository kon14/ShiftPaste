use sqlx::PgExecutor;
use uuid::Uuid;

use crate::prelude::*;

pub async fn archive_user_snippets<'a>(
    db: impl PgExecutor<'a>,
    user_id: Uuid,
) -> Result<(), AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to archive user snippets!";

    sqlx::query!(
        r#"
        SELECT archive_user_snippets($1)
        "#,
        user_id,
    )
    .execute(db)
    .await
    .map_err(|err| AppError::unauthorized_with_private(INTERNAL_ERR_STR, err.to_string()))?;

    Ok(())
}
