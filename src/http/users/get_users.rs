use axum::{
    extract::{Extension, Query, State},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

use crate::common::params::PaginationParams;
use crate::common::state::AppState;
use crate::domain as dmn;
use crate::domain::types::{JsonWebTokenData, User};
use crate::prelude::*;

#[derive(Serialize, ToSchema)]
pub struct GetUsersHttpResponse {
    pub users: Vec<User>,
    pub count: u32,
}

/// Retrieves multiple Users.
#[utoipa::path(
    get,
    path = "/users",
    params(
        PaginationParams,
    ),
    responses(
        (status = 200, description = "Success", body = GetUsersHttpResponse),
        (status = 500, description = "Failure"),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_users(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
    Extension(auth_user): Extension<User>,
    Extension(_auth_token): Extension<JsonWebTokenData>,
) -> Result<Json<GetUsersHttpResponse>, AppError> {
    let AppState { db } = state;

    let users = dmn::users::get_users(&db, &pagination, auth_user.id).await?;
    Ok(Json(users.into()))
}

impl From<dmn::users::GetUsersDmnResponse> for GetUsersHttpResponse {
    fn from(dmn_res: dmn::users::GetUsersDmnResponse) -> Self {
        GetUsersHttpResponse {
            users: dmn_res.users,
            count: dmn_res.count,
        }
    }
}
