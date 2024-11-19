use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::common::state::AppState;
use crate::domain as dmn;
use crate::domain::types::{AccessToken, AuthTokenPair, Email, RefreshToken};
use crate::prelude::*;

#[derive(Deserialize, ToSchema)]
pub struct UserLoginHttpParams {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct UserLoginHttpResponse {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}

/// Authenticates a User.
#[utoipa::path(
    post,
    path = "/auth/tokens",
    responses(
        (status = 200, description = "Success", body = AuthTokenPair),
        (status = 500, description = "Failure"),
    ),
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<UserLoginHttpParams>,
) -> Result<Json<AuthTokenPair>, AppError> {
    let AppState { db } = state;

    let payload = payload.try_into()?;
    let tokens = dmn::auth::login(&db, payload).await?;

    Ok(Json(tokens.into()))
}

impl TryFrom<UserLoginHttpParams> for dmn::auth::UserLoginDmnParams {
    type Error = AppError;

    fn try_from(http_args: UserLoginHttpParams) -> Result<Self, Self::Error> {
        Ok(dmn::auth::UserLoginDmnParams {
            email: Email::try_from_user_input(&http_args.email)?,
            password: http_args.password,
        })
    }
}
