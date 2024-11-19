use axum::{extract::State, Extension, Json};
use serde::Serialize;
use utoipa::ToSchema;

use crate::common::state::AppState;
use crate::domain as dmn;
use crate::domain::types::{
    AccessTokenPublic, AuthTokenPair, JsonWebTokenData, RefreshTokenPublic, User,
};
use crate::prelude::*;

#[derive(Serialize, ToSchema)]
pub struct GetAuthTokensHttpResponse {
    tokens: Vec<GetAuthTokenHttpPair>,
}

#[derive(Serialize, ToSchema)]
struct GetAuthTokenHttpPair {
    access_token: AccessTokenPublic,
    refresh_token: RefreshTokenPublic,
}

/// Retrieves authenticated User's auth token pairs.
#[utoipa::path(
    get,
    path = "/auth/tokens",
    responses(
        (status = 200, description = "Success", body = GetAuthTokensHttpResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Failure"),
    ),
    security(
        ("bearerAuth" = [])
    ),
)]
pub async fn get_tokens(
    State(state): State<AppState>,
    Extension(auth_user): Extension<User>,
    Extension(_auth_token): Extension<JsonWebTokenData>,
) -> Result<Json<GetAuthTokensHttpResponse>, AppError> {
    let AppState { db } = state;

    let tokens_db = dmn::auth::get_tokens(&db, auth_user.id).await?;
    let tokens = tokens_db.into_iter().map(|pair| pair.into()).collect();

    Ok(Json(GetAuthTokensHttpResponse { tokens }))
}

impl From<AuthTokenPair> for GetAuthTokenHttpPair {
    fn from(pair: AuthTokenPair) -> Self {
        GetAuthTokenHttpPair {
            access_token: pair.access_token.into(),
            refresh_token: pair.refresh_token.into(),
        }
    }
}
