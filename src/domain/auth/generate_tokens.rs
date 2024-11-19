use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::db;
use crate::db::DbExecutor;
use crate::domain::types::{AccessToken, AuthTokenPair, JsonWebTokenData, RefreshToken};
use crate::prelude::*;

pub async fn generate_tokens<'a>(
    db: DbExecutor<'_, '_>,
    user_id: Uuid,
) -> Result<AuthTokenPair, AppError>
where
    AuthTokenPair: 'static,
{
    const INTERNAL_ERR_STR: &str = "Failed to generate auth tokens!";

    let access_token = JsonWebTokenData::new_access(user_id);
    let refresh_token = JsonWebTokenData::new_refresh(user_id);

    let access_token = AccessToken::from_jwt(&access_token)?;
    let refresh_token = RefreshToken::from_jwt(&refresh_token, &access_token)?;
    let tokens = AuthTokenPair {
        access_token,
        refresh_token,
    };

    let tokens = match db {
        DbExecutor::Pool(pool) => {
            let mut tx = pool.begin().await.map_err(|err| {
                AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string())
            })?;
            let tokens = store_db_data(&mut tx, tokens).await?;
            tx.commit().await.map_err(|err| {
                AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string())
            })?;
            tokens
        }
        DbExecutor::Transaction(tx) => {
            let tokens = store_db_data(tx, tokens).await?;
            tokens
        }
    };

    Ok(tokens)
}

async fn store_db_data(
    tx: &mut Transaction<'_, Postgres>,
    tokens: AuthTokenPair,
) -> Result<AuthTokenPair, AppError> {
    let access_token = db::auth::create_access_token(tx.as_mut(), tokens.access_token).await?;
    let refresh_token = db::auth::create_refresh_token(tx.as_mut(), tokens.refresh_token).await?;
    Ok(AuthTokenPair {
        access_token,
        refresh_token,
    })
}
