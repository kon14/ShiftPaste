use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, ToSchema)]
pub struct Snippet {
    pub id: Uuid,
    pub data: SnippetData,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "variant")]
pub enum SnippetData {
    #[serde(rename = "text")]
    Text(TextSnippet),
    #[serde(rename = "url")]
    URL(UrlSnippet),
}

#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct TextSnippet {
    pub text: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct UrlSnippet {
    pub url: String,
}
