use sqlx::PgExecutor;

use crate::db::types::UserDb;
use crate::domain::types::{UniqueUserIdentifier, User};
use crate::prelude::*;

pub async fn get_user<'a>(
    db: impl PgExecutor<'a>,
    user_id: UniqueUserIdentifier,
) -> Result<User, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve user!";

    let (id, email) = match user_id {
        UniqueUserIdentifier::Id(id) => (Some(id), None),
        UniqueUserIdentifier::Email(ref email) => (None, Some(email.0.clone())),
    };
    sqlx::query_as!(
        UserDb,
        r#"
        SELECT
            id,
            email,
            created_at,
            updated_at
        FROM users
        WHERE id = $1 OR email = $2
        "#,
        id,
        email,
    )
    .fetch_one(db)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => not_found_err(&user_id),
        _ => AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()),
    })?
    .try_into()
}

pub fn not_found_err(user_id: &UniqueUserIdentifier) -> AppError {
    AppError::not_found(format!("User ({user_id}) doesn't exist!"))
}
