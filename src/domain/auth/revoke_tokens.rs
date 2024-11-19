use sqlx::PgPool;
use uuid::Uuid;

use crate::db;
use crate::domain::types::UniqueAccessTokenIdentifier;
use crate::prelude::*;

pub async fn revoke_tokens(
    db: &PgPool,
    access_token_id: UniqueAccessTokenIdentifier,
    authz_user_id: Uuid,
) -> Result<(), AppError> {
    // Authorization: Inner SQL query matches user ID
    db::auth::delete_token_pair(db, authz_user_id, access_token_id).await?;

    Ok(())
}
