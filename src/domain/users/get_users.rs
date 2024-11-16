use crate::common::params::PaginationParams;
use crate::db;
use crate::domain::types::User;
use crate::prelude::*;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

pub struct GetUsersDmnResponse {
    pub users: Vec<User>,
    pub count: u32,
}

pub async fn get_users(
    db: &PgPool,
    pagination: &PaginationParams,
    authz_user_id: Uuid,
) -> Result<GetUsersDmnResponse, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve users!";

    authorize(authz_user_id)?;

    let mut tx: Transaction<Postgres> = db
        .begin()
        .await
        .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;

    let users = db::users::get_users(tx.as_mut(), pagination).await?;
    let count = db::users::get_users_count(tx.as_mut()).await?;

    tx.commit()
        .await
        .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;
    Ok(GetUsersDmnResponse { users, count })
}

fn authorize(authz_user: Uuid) -> Result<(), AppError> {
    // TODO:
    // - admin-level authz access through through either
    //   a) secret token header
    //   b) admin role (db column)
    // - disable admin/all authz through build flag
    Ok(())
}
