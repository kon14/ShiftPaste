use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "snippet_variant")]
pub enum SnippetVariant {
    #[sqlx(rename = "TEXT")]
    Text,
    #[sqlx(rename = "URL")]
    URL,
}

pub struct SnippetDb {
    pub id: Uuid,
    pub variant: SnippetVariant,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct SnippetDataTextDb {
    pub snippet_id: Uuid,
    pub text: String,
}

pub struct SnippetDataUrlDb {
    pub snippet_id: Uuid,
    pub url: String,
}
