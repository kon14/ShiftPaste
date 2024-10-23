use sqlx::PgExecutor;

use crate::prelude::*;

pub async fn get_snippets_count<'a>(
    db: impl PgExecutor<'a>,
    archived_filter: Option<bool>,
) -> Result<u32, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve count of snippets!";

    sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) as "total_count!"
        FROM snippets
        WHERE
            ($1::boolean IS NULL OR archived = $1::boolean)
        "#,
        archived_filter,
    )
    .fetch_one(db)
    .await
    .map(|count| count as u32)
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}
