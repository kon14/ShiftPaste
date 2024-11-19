use sqlx::PgExecutor;

use crate::domain::types::RefreshToken;
use crate::prelude::*;

pub async fn create_refresh_token<'a>(
    db: impl PgExecutor<'a>,
    data: RefreshToken,
) -> Result<RefreshToken, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to create refresh token!";

    sqlx::query_as!(
        RefreshToken,
        r#"
        INSERT INTO refresh_tokens (
            id,
            user_id,
            access_token_id,
            jwt,
            expires_at
        )
        VALUES ($1, $2, $3, $4, $5)
        RETURNING
            id,
            user_id,
            access_token_id,
            jwt,
            expires_at
        "#,
        data.id,
        data.user_id,
        data.access_token_id,
        data.jwt,
        data.expires_at
    )
    .fetch_one(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}
