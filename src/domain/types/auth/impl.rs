use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use uuid::Uuid;

use super::{
    AccessToken, JsonWebTokenData, JsonWebTokenDataVariant, RefreshToken,
    UniqueAccessTokenIdentifier, UniqueRefreshTokenIdentifier,
};
use crate::common;
use crate::common::state::AppState;
use crate::domain as dmn;
use crate::domain::types::{AccessTokenPublic, RefreshTokenPublic};
use crate::http;
use crate::prelude::*;

impl JsonWebTokenData {
    pub fn new_access(user_id: Uuid) -> Self {
        let expiry_secs = common::utils::get_auth_access_token_duration_secs();
        let expiry = Utc::now() + Duration::seconds(expiry_secs as i64);
        JsonWebTokenData {
            id: Uuid::new_v4(),
            user_id,
            expires_at: expiry,
            variant: JsonWebTokenDataVariant::AccessToken,
        }
    }

    pub fn new_refresh(user_id: Uuid) -> Self {
        let expiry_secs = common::utils::get_auth_refresh_token_duration_secs();
        let expiry = Utc::now() + Duration::seconds(expiry_secs as i64);
        JsonWebTokenData {
            id: Uuid::new_v4(),
            user_id,
            expires_at: expiry,
            variant: JsonWebTokenDataVariant::RefreshToken,
        }
    }

    pub fn encode(&self) -> Result<String, AppError> {
        let jwt_secret = common::utils::get_auth_jwt_secret();
        let token = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )
        .map_err(|err| {
            AppError::internal_with_private("Failed to encode token!", err.to_string())
        })?;
        Ok(token)
    }

    pub fn decode(token: &str) -> Result<JsonWebTokenData, AppError> {
        let jwt_secret = common::utils::get_auth_jwt_secret();
        let token_data = decode::<JsonWebTokenData>(
            &token,
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|err| {
            AppError::internal_with_private("Failed to decode token!", err.to_string())
        })?;
        Ok(token_data.claims)
    }
}

impl AccessToken {
    pub fn from_jwt(jwt: &JsonWebTokenData) -> Result<Self, AppError> {
        let jwt_encoded = jwt.encode()?;
        Ok(AccessToken {
            id: jwt.id,
            user_id: jwt.user_id,
            jwt: jwt_encoded,
            expires_at: jwt.expires_at,
        })
    }
}

impl RefreshToken {
    pub fn from_jwt(jwt: &JsonWebTokenData, access_token: &AccessToken) -> Result<Self, AppError> {
        let jwt_encoded = jwt.encode()?;
        Ok(RefreshToken {
            id: jwt.id,
            user_id: jwt.user_id,
            access_token_id: access_token.id,
            jwt: jwt_encoded,
            expires_at: jwt.expires_at,
        })
    }
}

impl FromRequestParts<AppState> for JsonWebTokenData {
    type Rejection = AppError;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        parts: &'life0 mut Parts,
        state: &'life1 AppState,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Rejection>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            const UNAUTHORIZED_ERR_STR: &str = "Failed to authenticate user!";

            let uri = &parts.uri;
            let (auth_usage_ctx, token_name) = match uri.path() {
                http::RENEW_TOKENS_PATH => (JsonWebTokenDataVariant::RefreshToken, "refresh_token"),
                _ => (JsonWebTokenDataVariant::AccessToken, "access_token"),
            };

            let TypedHeader(Authorization(bearer)) = parts
                .extract::<TypedHeader<Authorization<Bearer>>>()
                .await
                .map_err(|err| {
                    AppError::unauthorized_with_private(UNAUTHORIZED_ERR_STR, err.to_string())
                })?;

            let token = bearer.token();
            if !token.starts_with("Bearer ") {
                return Err(AppError::unauthorized(format!(
                    r#"Invalid authorization header format! (example: "Bearer {token_name}")"#
                )));
            }
            let token = &token[7..];
            let auth_token = dmn::auth::authenticate(&state.db, token, auth_usage_ctx).await?;

            Ok(auth_token)
        })
    }
}

impl fmt::Display for UniqueAccessTokenIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UniqueAccessTokenIdentifier::Id(id) => write!(f, "{}", id),
            UniqueAccessTokenIdentifier::Jwt(jwt) => write!(f, "{}", jwt),
        }
    }
}

impl fmt::Display for UniqueRefreshTokenIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UniqueRefreshTokenIdentifier::Id(id) => write!(f, "{}", id),
            UniqueRefreshTokenIdentifier::Jwt(jwt) => write!(f, "{}", jwt),
            UniqueRefreshTokenIdentifier::AccessTokenId(access_token_id) => {
                write!(f, "{}", access_token_id)
            }
        }
    }
}

impl From<AccessToken> for JsonWebTokenData {
    fn from(token: AccessToken) -> Self {
        JsonWebTokenData {
            id: token.id,
            user_id: token.user_id,
            expires_at: token.expires_at,
            variant: JsonWebTokenDataVariant::AccessToken,
        }
    }
}

impl From<RefreshToken> for JsonWebTokenData {
    fn from(token: RefreshToken) -> Self {
        JsonWebTokenData {
            id: token.id,
            user_id: token.user_id,
            expires_at: token.expires_at,
            variant: JsonWebTokenDataVariant::RefreshToken,
        }
    }
}

impl From<AccessToken> for AccessTokenPublic {
    fn from(token: AccessToken) -> Self {
        AccessTokenPublic {
            id: token.id,
            user_id: token.user_id,
            expires_at: token.expires_at,
        }
    }
}

impl From<RefreshToken> for RefreshTokenPublic {
    fn from(token: RefreshToken) -> Self {
        RefreshTokenPublic {
            id: token.id,
            user_id: token.user_id,
            access_token_id: token.access_token_id,
            expires_at: token.expires_at,
        }
    }
}
