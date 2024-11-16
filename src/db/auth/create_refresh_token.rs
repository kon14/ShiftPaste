use chrono::{DateTime, Utc};
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::db::types::RefreshTokenDb;
use crate::prelude::*;

pub struct CreateRefreshTokenDbParams {
    pub user_id: Uuid,
    pub jwt: String,
    pub access_token_id: Uuid,
    pub expires_at: DateTime<Utc>,
}

pub async fn create_refresh_token<'a>(
    db: impl PgExecutor<'a>,
    data: CreateRefreshTokenDbParams,
) -> Result<RefreshTokenDb, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to create refresh token!";

    sqlx::query_as!(
        RefreshTokenDb,
        r#"
        INSERT INTO refresh_tokens (
            user_id,
            access_token_id,
            jwt,
            expires_at
        )
        VALUES ($1, $2, $3, $4)
        RETURNING
            id,
            user_id,
            access_token_id,
            jwt,
            expires_at
        "#,
        data.user_id,
        data.access_token_id,
        data.jwt,
        data.expires_at
    )
    .fetch_one(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}
