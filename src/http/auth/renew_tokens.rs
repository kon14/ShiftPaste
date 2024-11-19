use axum::{extract::State, Extension, Json};

use crate::common::state::AppState;
use crate::domain as dmn;
use crate::domain::types::{AuthTokenPair, JsonWebTokenData, UniqueRefreshTokenIdentifier, User};
use crate::prelude::*;

pub const PATH: &str = "/auth/tokens/renew";

/// Renews an auth token pair, revoking the old tokens. Accepts the current refresh token in the Authorization header!
#[utoipa::path(
    post,
    path = PATH,
    responses(
        (status = 200, description = "Success", body = AuthTokenPair),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Failure"),
    ),
    security(
        ("bearerAuth" = [])
    ),
)]
pub async fn renew_tokens(
    State(state): State<AppState>,
    Extension(_auth_user): Extension<User>,
    Extension(auth_token): Extension<JsonWebTokenData>,
) -> Result<Json<AuthTokenPair>, AppError> {
    let AppState { db } = state;

    let refresh_token_id = UniqueRefreshTokenIdentifier::Id(auth_token.id);
    let tokens = dmn::auth::renew_tokens(&db, refresh_token_id).await?;

    Ok(Json(tokens.into()))
}
