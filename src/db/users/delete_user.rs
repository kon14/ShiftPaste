use sqlx::PgExecutor;
use uuid::Uuid;

use crate::prelude::*;

pub async fn delete_user<'a>(db: impl PgExecutor<'a>, user_id: Uuid) -> Result<(), AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to delete user!";

    sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
        user_id,
    )
    .execute(db)
    .await
    .map(|_| ())
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => not_found_err(user_id),
        _ => AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()),
    })
}

pub fn not_found_err(user_id: Uuid) -> AppError {
    AppError::not_found(format!("User ({user_id}) doesn't exist!"))
}
