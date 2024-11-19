use sqlx::PgExecutor;

use crate::domain::types::AccessToken;
use crate::prelude::*;

pub async fn create_access_token<'a>(
    db: impl PgExecutor<'a>,
    data: AccessToken,
) -> Result<AccessToken, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to create access token!";

    sqlx::query_as!(
        AccessToken,
        r#"
        INSERT INTO access_tokens (
            id,
            user_id,
            jwt,
            expires_at
        )
        VALUES ($1, $2, $3, $4)
        RETURNING
            id,
            user_id,
            jwt,
            expires_at
        "#,
        data.id,
        data.user_id,
        data.jwt,
        data.expires_at
    )
    .fetch_one(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}
