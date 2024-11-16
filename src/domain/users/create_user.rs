use sqlx::PgPool;

use crate::common::crypto;
use crate::db;
use crate::domain::types::{Email, User};
use crate::prelude::*;

pub struct CreateUserDmnParams {
    pub email: Email,
    pub password: String,
}

pub async fn create_user(db: &PgPool, payload: CreateUserDmnParams) -> Result<User, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to create user!";

    let password_hash = crypto::hash_password(&payload.password)?;

    let email_registered = db::users::user_email_exists(db, &payload.email).await?;
    if email_registered {
        return Err(AppError::conflict("Email already registered!"));
    }

    let user = db::users::create_user(db, payload.email, password_hash)
        .await
        .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;

    Ok(user)
}
