use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::db;
use crate::db::users::delete_user::not_found_err;
use crate::prelude::*;

pub async fn delete_user(db: &PgPool, user_id: Uuid, authz_user_id: Uuid) -> Result<(), AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to delete user!";

    authorize(authz_user_id, user_id)?;

    let mut tx: Transaction<Postgres> = db
        .begin()
        .await
        .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;

    db::snippets::archive_user_snippets(tx.as_mut(), user_id).await?;
    db::users::delete_user(tx.as_mut(), user_id).await?;

    tx.commit()
        .await
        .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;
    Ok(())
}

fn authorize(authz_user_id: Uuid, target_user_id: Uuid) -> Result<(), AppError> {
    if authz_user_id == target_user_id {
        Ok(())
    } else {
        Err(not_found_err(target_user_id))
    }
}
