use sqlx::PgPool;
use uuid::Uuid;

use crate::db;
use crate::prelude::*;

pub async fn archive_snippet(db: &PgPool, snippet_id: Uuid) -> Result<(), AppError> {
    db::snippets::archive_snippet(db, snippet_id).await
}
