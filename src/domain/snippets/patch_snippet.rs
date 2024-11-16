use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::db;
use crate::db::types::SnippetVariant;
use crate::domain::types::{Snippet, SnippetData};
use crate::prelude::*;

pub struct PatchSnippetDmnParams {
    pub data: SnippetData,
}

pub async fn patch_snippet(
    db: &PgPool,
    snippet_id: Uuid,
    payload: PatchSnippetDmnParams,
) -> Result<Snippet, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to patch snippet!";

    let mut tx: Transaction<Postgres> = db
        .begin()
        .await
        .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;
    let snippet = db::snippets::get_snippet(tx.as_mut(), snippet_id, Some(false)).await?;

    let data: SnippetData;
    let next_variant = payload.data.get_variant();
    if next_variant == snippet.variant {
        data = match payload.data {
            SnippetData::Text(data) => {
                let text_data =
                    db::snippets::patch_snippet_data_text(tx.as_mut(), snippet.id, data).await?;
                text_data.into()
            }
            SnippetData::URL(data) => {
                let url_data =
                    db::snippets::patch_snippet_data_url(tx.as_mut(), snippet.id, data).await?;
                url_data.into()
            }
        };
    } else {
        db::snippets::patch_snippet(tx.as_mut(), snippet.id, next_variant).await?;
        match snippet.variant {
            SnippetVariant::Text => {
                db::snippets::delete_snippet_data_text(tx.as_mut(), snippet.id).await?
            }
            SnippetVariant::URL => {
                db::snippets::delete_snippet_data_url(tx.as_mut(), snippet.id).await?
            }
        };
        data = match payload.data {
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
    }

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
