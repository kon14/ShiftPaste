use sqlx::{PgPool, Postgres, Transaction};

use crate::db;
use crate::domain as dmn;
use crate::domain::types::{
    AccessToken, Email, JsonWebTokenData, RefreshToken, UniqueUserIdentifier,
};
use crate::prelude::*;

pub struct GenerateAuthTokensDmnParams {
    pub email: Email,
    pub password: String,
}

pub struct GenerateAuthTokensDmnResponse {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}

pub async fn generate_tokens(
    db: &PgPool,
    payload: GenerateAuthTokensDmnParams,
) -> Result<GenerateAuthTokensDmnResponse, AppError> {
    const UNAUTHORIZED_ERR_STR: &str = "Failed to authenticate user!";

    let user = db::users::get_user(db, UniqueUserIdentifier::Email(payload.email))
        .await
        .map_err(|err| {
            AppError::unauthorized_with_private(UNAUTHORIZED_ERR_STR, err.to_string())
        })?;
    dmn::auth::validate_user_password(db, user.id, &payload.password)
        .await
        .map_err(|err| {
            AppError::unauthorized_with_private(UNAUTHORIZED_ERR_STR, err.to_string())
        })?;

    let access_token = JsonWebTokenData::new_access(user.id);
    let refresh_token = JsonWebTokenData::new_refresh(user.id);
    let access_token_jwt = access_token.encode()?;
    let refresh_token_jwt = refresh_token.encode()?;

    let mut tx: Transaction<Postgres> = db
        .begin()
        .await
        .map_err(|err| AppError::internal_with_private(UNAUTHORIZED_ERR_STR, err.to_string()))?;
    let access_token_data = db::auth::CreateAccessTokenDbParams {
        user_id: user.id,
        jwt: access_token_jwt,
        expires_at: access_token.expires_at,
    };
    let access_token_db = db::auth::create_access_token(tx.as_mut(), access_token_data).await?;
    let refresh_token_data = db::auth::CreateRefreshTokenDbParams {
        user_id: user.id,
        jwt: refresh_token_jwt,
        access_token_id: access_token_db.id,
        expires_at: access_token.expires_at,
    };
    let refresh_token_db = db::auth::create_refresh_token(tx.as_mut(), refresh_token_data).await?;
    tx.commit()
        .await
        .map_err(|err| AppError::internal_with_private(UNAUTHORIZED_ERR_STR, err.to_string()))?;

    Ok(GenerateAuthTokensDmnResponse {
        access_token: access_token_db,
        refresh_token: refresh_token_db,
    })
}
