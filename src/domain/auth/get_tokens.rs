use sqlx::PgPool;
use uuid::Uuid;

use crate::db;
use crate::domain::types::AuthTokenPair;
use crate::prelude::*;

pub async fn get_tokens(db: &PgPool, user_id: Uuid) -> Result<Vec<AuthTokenPair>, AppError> {
    let tokens = db::auth::get_user_tokens(db, user_id).await?;
    Ok(tokens)
}
