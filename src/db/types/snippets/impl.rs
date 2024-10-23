use super::types::*;
use crate::domain::types::{SnippetData, TextSnippet, UrlSnippet};

impl SnippetData {
    pub fn get_variant(&self) -> SnippetVariant {
        match self {
            SnippetData::Text(_) => SnippetVariant::Text,
            SnippetData::URL(_) => SnippetVariant::URL,
        }
    }
}

impl From<TextSnippet> for SnippetData {
    fn from(data: TextSnippet) -> Self {
        SnippetData::Text(data)
    }
}

impl From<UrlSnippet> for SnippetData {
    fn from(data: UrlSnippet) -> Self {
        SnippetData::URL(data)
    }
}
