use sqlx::PgExecutor;

use crate::prelude::*;

pub async fn get_users_count<'a>(db: impl PgExecutor<'a>) -> Result<u32, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve count of users!";

    sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) as "total_count!"
        FROM users
        "#,
    )
    .fetch_one(db)
    .await
    .map(|count| count as u32)
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}
