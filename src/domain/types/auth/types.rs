use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonWebToken {
    #[serde(rename = "sub")]
    pub user_id: Uuid,
    #[serde(rename = "exp", with = "chrono::serde::ts_seconds")]
    pub expires_at: DateTime<Utc>,
    #[serde(rename = "type")]
    pub variant: JsonWebTokenVariant,
}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[sqlx(type_name = "auth_token_variant")]
pub enum JsonWebTokenVariant {
    #[serde(rename = "access")]
    AccessToken,
    #[serde(rename = "refresh")]
    RefreshToken,
}

pub enum UniqueAccessTokenIdentifier {
    Id(Uuid),
    Jwt(String),
}

pub enum UniqueRefreshTokenIdentifier {
    Id(Uuid),
    Jwt(String),
    AccessTokenId(Uuid),
}
