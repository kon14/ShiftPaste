use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonWebTokenData {
    #[serde(rename = "tokenId")]
    pub id: Uuid,
    #[serde(rename = "sub")]
    pub user_id: Uuid,
    #[serde(rename = "exp", with = "chrono::serde::ts_seconds")]
    pub expires_at: DateTime<Utc>,
    #[serde(rename = "type")]
    pub variant: JsonWebTokenDataVariant,
}

#[derive(Debug, Clone, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "auth_token_variant")]
pub enum JsonWebTokenDataVariant {
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AccessToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub jwt: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub access_token_id: Uuid,
    pub jwt: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AuthTokenPair {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AccessTokenPublic {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RefreshTokenPublic {
    pub id: Uuid,
    pub user_id: Uuid,
    pub access_token_id: Uuid,
    pub expires_at: DateTime<Utc>,
}
