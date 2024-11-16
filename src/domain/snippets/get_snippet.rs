use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::db;
use crate::db::types::SnippetVariant;
use crate::domain::types::{Snippet, SnippetData};
use crate::prelude::*;

pub async fn get_snippet(db: &PgPool, snippet_id: Uuid) -> Result<Snippet, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve snippet!";

    let mut tx: Transaction<Postgres> = db
        .begin()
        .await
        .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;
    let snippet = db::snippets::get_snippet(tx.as_mut(), snippet_id, Some(false)).await?;
    let data: SnippetData = match snippet.variant {
        SnippetVariant::Text => {
            let text_data = db::snippets::get_snippet_data_text(tx.as_mut(), snippet.id).await?;
            text_data.into()
        }
        SnippetVariant::URL => {
            let url_data = db::snippets::get_snippet_data_url(tx.as_mut(), snippet.id).await?;
            url_data.into()
        }
    };
    let snippet = Snippet {
        id: snippet.id,
        data,
        created_at: snippet.created_at,
        updated_at: snippet.updated_at,
    };
    tx.commit()
        .await
        .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;

    Ok(snippet)
}
