use chrono::{DateTime, Utc};
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::domain::types::{AccessToken, AuthTokenPair, RefreshToken};
use crate::prelude::*;

pub async fn get_user_tokens<'a>(
    db: impl PgExecutor<'a>,
    user_id: Uuid,
) -> Result<Vec<AuthTokenPair>, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve authentication tokens!";

    let tokens = sqlx::query_as!(
        GetUserTokensDbRowInner,
        r#"
        SELECT
            at.user_id as user_id,
            at.id as at_id,
            at.jwt as at_jwt,
            at.expires_at as at_expires_at,
            rt.id as rt_id,
            rt.jwt as rt_jwt,
            rt.expires_at as rt_expires_at
        FROM refresh_tokens rt
        INNER JOIN access_tokens at
        ON rt.access_token_id = at.id
        WHERE rt.user_id = $1
        ORDER BY at.expires_at DESC
        "#,
        user_id,
    )
    .fetch_all(db)
    .await
    .map_err(|err| match err {
        _ => AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()),
    })?
    .into_iter()
    .map(|token| token.into())
    .collect();

    Ok(tokens)
}

struct GetUserTokensDbRowInner {
    user_id: Uuid,
    at_id: Uuid,
    at_jwt: String,
    at_expires_at: DateTime<Utc>,
    rt_id: Uuid,
    rt_jwt: String,
    rt_expires_at: DateTime<Utc>,
}

impl From<GetUserTokensDbRowInner> for AuthTokenPair {
    fn from(db_res: GetUserTokensDbRowInner) -> Self {
        AuthTokenPair {
            access_token: AccessToken {
                id: db_res.at_id,
                user_id: db_res.user_id,
                jwt: db_res.at_jwt,
                expires_at: db_res.at_expires_at,
            },
            refresh_token: RefreshToken {
                id: db_res.rt_id,
                user_id: db_res.user_id,
                access_token_id: db_res.at_id,
                jwt: db_res.rt_jwt,
                expires_at: db_res.rt_expires_at,
            },
        }
    }
}
