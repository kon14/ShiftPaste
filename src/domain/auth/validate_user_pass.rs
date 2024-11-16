use sqlx::PgPool;
use uuid::Uuid;

use crate::common::crypto;
use crate::db;
use crate::prelude::*;

pub async fn validate_user_password(
    db: &PgPool,
    user_id: Uuid,
    password: &str,
) -> Result<(), AppError> {
    const UNAUTHORIZED_ERR_STR: &str = "Failed to validate user password!";

    let stored_hash = db::users::get_user_password_hash(db, user_id).await?;

    let is_valid = crypto::verify_password(password, &stored_hash).map_err(|err| {
        AppError::unauthorized_with_private(UNAUTHORIZED_ERR_STR, err.to_string())
    })?;
    match is_valid {
        true => Ok(()),
        false => Err(AppError::unauthorized(UNAUTHORIZED_ERR_STR)),
    }
}
