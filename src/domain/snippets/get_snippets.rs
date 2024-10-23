use sqlx::{PgPool, Postgres, Transaction};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::common::params::PaginationParams;
use crate::db;
use crate::db::types::SnippetVariant;
use crate::domain::types::{Snippet, SnippetData};
use crate::prelude::*;

pub struct GetSnippetsDmnResponse {
    pub snippets: Vec<Snippet>,
    pub count: u32,
}

pub async fn get_snippets(
    db: &PgPool,
    pagination: &PaginationParams,
) -> Result<GetSnippetsDmnResponse, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve snippets!";

    let mut tx: Transaction<Postgres> = db
        .begin()
        .await
        .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;

    // Data Fetching
    let snippets_db = db::snippets::get_snippets(tx.as_mut(), pagination, Some(false)).await?;
    let count = db::snippets::get_snippets_count(tx.as_mut(), Some(false)).await?;

    let (snippet_ids_text, snippet_ids_url) = snippets_db.iter().fold(
        (HashSet::new(), HashSet::new()),
        |(mut text_set, mut url_set), snippet| {
            match snippet.variant {
                SnippetVariant::Text => {
                    text_set.insert(snippet.id);
                }
                SnippetVariant::URL => {
                    url_set.insert(snippet.id);
                }
            }
            (text_set, url_set)
        },
    );

    // Data Mapping
    let mut snippets_data_map: HashMap<Uuid, SnippetData> =
        HashMap::with_capacity(snippets_db.len());
    if snippet_ids_text.len() > 0 {
        let snippets_data_text =
            db::snippets::get_snippets_data_text(tx.as_mut(), &snippet_ids_text).await?;
        snippets_data_map.extend(
            snippets_data_text
                .into_iter()
                .map(|snippet_text| (snippet_text.snippet_id, snippet_text.into())),
        );
    }
    if snippet_ids_url.len() > 0 {
        let snippets_data_url =
            db::snippets::get_snippets_data_url(tx.as_mut(), &snippet_ids_url).await?;
        snippets_data_map.extend(
            snippets_data_url
                .into_iter()
                .map(|snippet_url| (snippet_url.snippet_id, snippet_url.into())),
        );
    }
    let mut missing_data_ids: HashSet<Uuid> = HashSet::new();
    let snippets = snippets_db
        .into_iter()
        .filter_map(|snippet_db| {
            let snippet_data = snippets_data_map.remove(&snippet_db.id);
            if let Some(snippet_data) = snippet_data {
                Some(Snippet {
                    id: snippet_db.id,
                    data: snippet_data,
                    created_at: snippet_db.created_at,
                    updated_at: snippet_db.updated_at,
                })
            } else {
                missing_data_ids.insert(snippet_db.id);
                None
            }
        })
        .collect();

    if missing_data_ids.len() > 0 {
        return Err(AppError::internal_with_private(
            "Couldn't retrieve snippet variant data!",
            format!(
                "Couldn't retrieve snippet variant data for the following IDs: {}",
                missing_data_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            ),
        ));
    }
    tx.commit()
        .await
        .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;
    Ok(GetSnippetsDmnResponse { snippets, count })
}
