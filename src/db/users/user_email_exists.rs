use sqlx::PgExecutor;

use crate::domain::types::Email;
use crate::prelude::*;

pub async fn user_email_exists<'a>(
    db: impl PgExecutor<'a>,
    email: &Email,
) -> Result<bool, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to check user email availability!";

    sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM users
            WHERE email = $1
        )
        "#,
        email,
    )
    .fetch_one(db)
    .await
    .map(|exists| exists.unwrap_or(false))
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}
