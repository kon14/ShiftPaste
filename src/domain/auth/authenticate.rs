use chrono::Timelike; // TODO: UniqueUserIdentifier
use sqlx::PgPool;

use crate::db;
use crate::domain::types::{JsonWebToken, UniqueAccessTokenIdentifier};
use crate::prelude::*;

pub async fn authenticate(db: &PgPool, access_token: &str) -> Result<JsonWebToken, AppError> {
    const UNAUTHORIZED_ERR_STR: &str = "Failed to authenticate user!";

    let token_id = UniqueAccessTokenIdentifier::Jwt(access_token.to_string());
    let req_access_token = JsonWebToken::decode(access_token).map_err(|err| {
        let decode_err = match &err.private_info {
            Some(private_info) => format!("{} {}", err.public_info, private_info),
            None => err.public_info.clone(),
        };
        AppError::unauthorized_with_private(UNAUTHORIZED_ERR_STR, decode_err)
    })?;

    // Retrieve non-revoked token from persistent storage.
    let token = db::auth::get_access_token(db, token_id)
        .await
        .map_err(|err| err.reword(UNAUTHORIZED_ERR_STR.to_string()))?;
    // Fetch implicitly guarantees user existence (users->access_tokens ON DELETE CASCADE).
    if token.user_id != req_access_token.user_id {
        return Err(AppError::unauthorized_with_private(
            UNAUTHORIZED_ERR_STR,
            "Token identity mismatch!",
        ));
    }
    let expires_at = token.expires_at.with_nanosecond(0).unwrap();
    if expires_at != req_access_token.expires_at {
        return Err(AppError::unauthorized_with_private(
            UNAUTHORIZED_ERR_STR,
            "Token expiry mismatch!",
        ));
    }

    Ok(req_access_token)
}
