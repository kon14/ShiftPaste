use sqlx::PgPool;

use crate::db;
use crate::db::DbExecutor;
use crate::domain as dmn;
use crate::domain::types::{AuthTokenPair, Email, UniqueUserIdentifier};
use crate::prelude::*;

pub struct UserLoginDmnParams {
    pub email: Email,
    pub password: String,
}

pub async fn login(db: &PgPool, payload: UserLoginDmnParams) -> Result<AuthTokenPair, AppError> {
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

    dmn::auth::generate_tokens(DbExecutor::Pool(db), user.id)
        .await
        .map_err(|err| err.reword(UNAUTHORIZED_ERR_STR.to_string()))
}
