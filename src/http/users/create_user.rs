use axum::{extract::State, Json};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::common::state::AppState;
use crate::domain as dmn;
use crate::domain::types::{Email, User};
use crate::prelude::*;

#[derive(Deserialize, ToSchema)]
pub struct CreateUserHttpParams {
    pub email: String,
    pub password: String,
}

/// Registers a new User.
#[utoipa::path(
    post,
    path = "/users",
    responses(
        (status = 200, description = "Success", body = User),
        (status = 500, description = "Failure"),
    ),
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserHttpParams>,
) -> Result<Json<User>, AppError> {
    let AppState { db } = state;

    let payload = payload.try_into()?;
    let user = dmn::users::create_user(&db, payload).await?;
    Ok(Json(user))
}

impl TryFrom<CreateUserHttpParams> for dmn::users::CreateUserDmnParams {
    type Error = AppError;

    fn try_from(http_args: CreateUserHttpParams) -> Result<Self, Self::Error> {
        Ok(dmn::users::CreateUserDmnParams {
            email: Email::try_from_user_input(&http_args.email)?,
            password: http_args.password,
        })
    }
}
