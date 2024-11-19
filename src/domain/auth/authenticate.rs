use chrono::Timelike;
use sqlx::PgPool;

use crate::db;
use crate::domain::types::{
    JsonWebTokenData, JsonWebTokenDataVariant, UniqueAccessTokenIdentifier,
    UniqueRefreshTokenIdentifier,
};
use crate::prelude::*;

pub async fn authenticate(
    db: &PgPool,
    auth_token: &str,
    usage_ctx: JsonWebTokenDataVariant,
) -> Result<JsonWebTokenData, AppError> {
    const UNAUTHORIZED_ERR_STR: &str = "Failed to authenticate user!";

    let req_auth_token = JsonWebTokenData::decode(auth_token).map_err(|err| {
        let decode_err = match &err.private_info {
            Some(private_info) => format!("{} {}", err.public_info, private_info),
            None => err.public_info.clone(),
        };
        AppError::unauthorized_with_private(UNAUTHORIZED_ERR_STR, decode_err)
    })?;

    if req_auth_token.variant != usage_ctx {
        return Err(AppError::bad_request(
            "Invalid authentication token variant provided!",
        ));
    }

    // Retrieve non-revoked token from persistent storage.
    // Fetch implicitly guarantees user existence (db record cascade).
    let (token_user_id, token_expires_at) = match req_auth_token.variant {
        JsonWebTokenDataVariant::AccessToken => {
            let token_id = UniqueAccessTokenIdentifier::Jwt(auth_token.to_string());
            let token = db::auth::get_access_token(db, token_id)
                .await
                .map_err(|err| err.reword(UNAUTHORIZED_ERR_STR.to_string()))?;
            (token.user_id, token.expires_at)
        }
        JsonWebTokenDataVariant::RefreshToken => {
            let token_id = UniqueRefreshTokenIdentifier::Jwt(auth_token.to_string());
            let token = db::auth::get_refresh_token(db, token_id)
                .await
                .map_err(|err| err.reword(UNAUTHORIZED_ERR_STR.to_string()))?;
            (token.user_id, token.expires_at)
        }
    };

    if token_user_id != req_auth_token.user_id {
        return Err(AppError::unauthorized_with_private(
            UNAUTHORIZED_ERR_STR,
            "Token identity mismatch!",
        ));
    }
    let expires_at = token_expires_at.with_nanosecond(0).unwrap();
    if expires_at != req_auth_token.expires_at {
        return Err(AppError::unauthorized_with_private(
            UNAUTHORIZED_ERR_STR,
            "Token expiry mismatch!",
        ));
    }

    Ok(req_auth_token)
}
