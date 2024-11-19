use sqlx::PgExecutor;
use uuid::Uuid;

use crate::domain::types::UniqueAccessTokenIdentifier;
use crate::prelude::*;

pub async fn delete_token_pair<'a>(
    db: impl PgExecutor<'a>,
    user_id: Uuid,
    access_token_id: UniqueAccessTokenIdentifier,
) -> Result<(), AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to delete auth token pair!";

    let (id, jwt) = match access_token_id {
        UniqueAccessTokenIdentifier::Id(id) => (Some(id), None),
        UniqueAccessTokenIdentifier::Jwt(ref jwt) => (None, Some(jwt)),
    };
    // Auto-cascades related refresh_tokens entries
    sqlx::query!(
        r#"
        DELETE FROM access_tokens
        WHERE
            user_id = $1 AND
            (id = $2 OR jwt = $3)
        "#,
        user_id,
        id,
        jwt,
    )
    .execute(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
    .and_then(|result| match result.rows_affected() {
        0 => Err(AppError::not_found(
            "Auth token pair doesn't exist!".to_string(),
        )),
        _ => Ok(()),
    })
}
