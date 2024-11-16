use sqlx::PgExecutor;

use crate::db::types::UserDb;
use crate::domain::types::{Email, User};
use crate::prelude::*;

pub async fn create_user<'a>(
    db: impl PgExecutor<'a>,
    email: Email,
    password_hash: String,
) -> Result<User, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to create user!";

    sqlx::query_as!(
        UserDb,
        r#"
        INSERT INTO users (
            email,
            password_hash
        )
        VALUES ($1, $2)
        RETURNING
            id,
            email,
            created_at,
            updated_at
        "#,
        email as Email,
        password_hash,
    )
    .fetch_one(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?
    .try_into()
}
