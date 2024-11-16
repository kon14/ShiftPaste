use sqlx::PgExecutor;

use crate::db::types::RefreshTokenDb;
use crate::domain::types::UniqueRefreshTokenIdentifier;
use crate::prelude::*;

pub async fn get_refresh_token<'a>(
    db: impl PgExecutor<'a>,
    refresh_token_id: UniqueRefreshTokenIdentifier,
) -> Result<RefreshTokenDb, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve refresh token!";

    let (id, jwt, access_token_id) = match refresh_token_id {
        UniqueRefreshTokenIdentifier::Id(id) => (Some(id), None, None),
        UniqueRefreshTokenIdentifier::Jwt(ref jwt) => (None, Some(jwt), None),
        UniqueRefreshTokenIdentifier::AccessTokenId(access_token_id) => {
            (None, None, Some(access_token_id))
        }
    };
    sqlx::query_as!(
        RefreshTokenDb,
        r#"
        SELECT
            id,
            user_id,
            access_token_id,
            jwt,
            expires_at
        FROM refresh_tokens
        WHERE
            id = $1 OR
            jwt = $2 OR
            access_token_id = $3
        "#,
        id,
        jwt,
        access_token_id,
    )
    .fetch_one(db)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => {
            AppError::not_found(format!("RefreshToken ({refresh_token_id}) doesn't exist!"))
        }
        _ => AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()),
    })
}
