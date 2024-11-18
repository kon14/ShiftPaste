use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::common::state::AppState;
use crate::domain as dmn;
use crate::domain::types::{AccessToken, Email, RefreshToken};
use crate::prelude::*;

#[derive(Deserialize, ToSchema)]
pub struct GenerateAuthTokensHttpParams {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct GenerateAuthTokensHttpResponse {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}

/// Authenticates a User.
#[utoipa::path(
    post,
    path = "/auth/tokens",
    responses(
        (status = 200, description = "Success", body = GenerateAuthTokensHttpResponse),
        (status = 500, description = "Failure"),
    ),
)]
pub async fn generate_tokens(
    State(state): State<AppState>,
    Json(payload): Json<GenerateAuthTokensHttpParams>,
) -> Result<Json<GenerateAuthTokensHttpResponse>, AppError> {
    let AppState { db } = state;

    let payload = payload.try_into()?;
    let tokens = dmn::auth::generate_tokens(&db, payload).await?;

    Ok(Json(tokens.into()))
}

impl TryFrom<GenerateAuthTokensHttpParams> for dmn::auth::GenerateAuthTokensDmnParams {
    type Error = AppError;

    fn try_from(http_args: GenerateAuthTokensHttpParams) -> Result<Self, Self::Error> {
        Ok(dmn::auth::GenerateAuthTokensDmnParams {
            email: Email::try_from_user_input(&http_args.email)?,
            password: http_args.password,
        })
    }
}

impl From<dmn::auth::GenerateAuthTokensDmnResponse> for GenerateAuthTokensHttpResponse {
    fn from(dmn_res: dmn::auth::GenerateAuthTokensDmnResponse) -> Self {
        GenerateAuthTokensHttpResponse {
            access_token: dmn_res.access_token,
            refresh_token: dmn_res.refresh_token,
        }
    }
}
