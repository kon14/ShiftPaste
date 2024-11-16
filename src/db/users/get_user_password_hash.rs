use sqlx::PgExecutor;
use uuid::Uuid;

use crate::prelude::*;

pub async fn get_user_password_hash<'a>(
    db: impl PgExecutor<'a>,
    user_id: Uuid,
) -> Result<String, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve user password hash!";

    sqlx::query_scalar!(
        r#"
        SELECT password_hash
        FROM users
        WHERE id = $1
        "#,
        user_id,
    )
    .fetch_one(db)
    .await
    .map_err(|err| AppError::unauthorized_with_private(INTERNAL_ERR_STR, err.to_string()))
}
