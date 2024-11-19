use sqlx::{PgPool, Postgres, Transaction};

use crate::db;
use crate::db::DbExecutor;
use crate::domain as dmn;
use crate::domain::types::{
    AuthTokenPair, UniqueAccessTokenIdentifier, UniqueRefreshTokenIdentifier,
};
use crate::prelude::*;

pub async fn renew_tokens(
    db: &PgPool,
    refresh_token_id: UniqueRefreshTokenIdentifier,
) -> Result<AuthTokenPair, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to renew auth tokens!";

    let refresh_token = db::auth::get_refresh_token(db, refresh_token_id)
        .await
        .map_err(|err| {
            AppError::internal_with_private(INTERNAL_ERR_STR.to_string(), err.to_string())
        })?;

    let mut tx: Transaction<Postgres> = db
        .begin()
        .await
        .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;
    let tokens =
        dmn::auth::generate_tokens(DbExecutor::Transaction(&mut tx), refresh_token.user_id)
            .await
            .map_err(|err| {
                AppError::internal_with_private(INTERNAL_ERR_STR.to_string(), err.to_string())
            })?;
    db::auth::delete_token_pair(
        tx.as_mut(),
        refresh_token.user_id,
        UniqueAccessTokenIdentifier::Id(refresh_token.access_token_id),
    )
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;
    tx.commit()
        .await
        .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;

    Ok(tokens)
}
