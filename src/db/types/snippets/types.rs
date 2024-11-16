use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Eq, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "snippet_variant")]
pub enum SnippetVariant {
    #[sqlx(rename = "TEXT")]
    Text,
    #[sqlx(rename = "URL")]
    URL,
}
