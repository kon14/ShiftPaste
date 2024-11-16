use sqlx::{PgPool, Postgres, Transaction};

use crate::db;
use crate::domain::types::{Snippet, SnippetData};
use crate::prelude::*;

pub struct CreateSnippetDmnParams {
    pub data: SnippetData,
}

pub async fn create_snippet(
    db: &PgPool,
    payload: CreateSnippetDmnParams,
) -> Result<Snippet, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to create snippet!";

    let variant = payload.data.get_variant();
    let mut tx: Transaction<Postgres> = db
        .begin()
        .await
        .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;
    let snippet = db::snippets::create_snippet(tx.as_mut(), variant).await?;
    let data: SnippetData = match payload.data {
        SnippetData::Text(data) => {
            let text_data =
                db::snippets::create_snippet_data_text(tx.as_mut(), snippet.id, data).await?;
            text_data.into()
        }
        SnippetData::URL(data) => {
            let url_data =
                db::snippets::create_snippet_data_url(tx.as_mut(), snippet.id, data).await?;
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
