use sqlx::PgExecutor;
use uuid::Uuid;

use crate::prelude::*;

pub async fn delete_snippet_data_url<'a>(
    db: impl PgExecutor<'a>,
    snippet_id: &Uuid,
) -> Result<(), AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to delete url snippet data!";

    sqlx::query!(
        r#"
        DELETE FROM snippets_data_url
        WHERE snippet_id = $1
        "#,
        snippet_id,
    )
    .execute(db)
    .await
    .map(|_| ())
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => {
            AppError::not_found(format!("Snippet ({snippet_id}) doesn't exist!"))
        }
        _ => AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()),
    })
}
