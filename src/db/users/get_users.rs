use sqlx::PgExecutor;

use crate::common::params::PaginationParams;
use crate::db::types::UserDb;
use crate::domain::types::User;
use crate::prelude::*;

pub async fn get_users<'a>(
    db: impl PgExecutor<'a>,
    pagination: &PaginationParams,
) -> Result<Vec<User>, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve snippets!";

    sqlx::query_as!(
        UserDb,
        r#"
        SELECT
            id,
            email,
            created_at,
            updated_at
        FROM users
        ORDER BY id ASC
        OFFSET $1
        LIMIT $2
        "#,
        pagination.skip as i64,
        pagination.limit as i64,
    )
    .fetch_all(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?
    .into_iter()
    .map(|user| user.try_into())
    .collect()
}
