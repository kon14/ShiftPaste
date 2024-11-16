use chrono::{DateTime, Utc};
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::db::types::AccessTokenDb;
use crate::prelude::*;

pub struct CreateAccessTokenDbParams {
    pub user_id: Uuid,
    pub jwt: String,
    pub expires_at: DateTime<Utc>,
}

pub async fn create_access_token<'a>(
    db: impl PgExecutor<'a>,
    data: CreateAccessTokenDbParams,
) -> Result<AccessTokenDb, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to create access token!";

    sqlx::query_as!(
        AccessTokenDb,
        r#"
        INSERT INTO access_tokens (
            user_id,
            jwt,
            expires_at
        )
        VALUES ($1, $2, $3)
        RETURNING
            id,
            user_id,
            jwt,
            expires_at
        "#,
        data.user_id,
        data.jwt,
        data.expires_at
    )
    .fetch_one(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}
