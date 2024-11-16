use sqlx::PgPool;
use uuid::Uuid;

use crate::db;
use crate::db::users::get_user::not_found_err;
use crate::domain::types::{UniqueUserIdentifier, User};
use crate::prelude::*;

pub async fn get_user(db: &PgPool, user_id: Uuid, authz_user_id: Uuid) -> Result<User, AppError> {
    authorize(authz_user_id, user_id)?;

    let target_user = UniqueUserIdentifier::Id(user_id);
    let user = db::users::get_user(db, target_user).await?;
    Ok(user)
}

fn authorize(authz_user_id: Uuid, target_user_id: Uuid) -> Result<(), AppError> {
    if authz_user_id == target_user_id {
        Ok(())
    } else {
        Err(not_found_err(&UniqueUserIdentifier::Id(target_user_id)))
    }
}
