use sqlx::PgExecutor;

use crate::db::types::AccessTokenDb;
use crate::domain::types::UniqueAccessTokenIdentifier;
use crate::prelude::*;

pub async fn get_access_token<'a>(
    db: impl PgExecutor<'a>,
    access_token_id: UniqueAccessTokenIdentifier,
) -> Result<AccessTokenDb, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve access token!";

    let (id, jwt) = match access_token_id {
        UniqueAccessTokenIdentifier::Id(id) => (Some(id), None),
        UniqueAccessTokenIdentifier::Jwt(ref jwt) => (None, Some(jwt)),
    };
    sqlx::query_as!(
        AccessTokenDb,
        r#"
        SELECT
            id,
            user_id,
            jwt,
            expires_at
        FROM access_tokens
        WHERE id = $1 OR jwt = $2
        "#,
        id,
        jwt,
    )
    .fetch_one(db)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => {
            AppError::not_found(format!("AccessToken ({access_token_id}) doesn't exist!"))
        }
        _ => AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()),
    })
}
