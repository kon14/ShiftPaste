use axum::{
    extract::{Path, State},
    Extension, Json,
};
use uuid::Uuid;

use crate::common::state::AppState;
use crate::domain as dmn;
use crate::domain::types::{JsonWebTokenData, User};
use crate::prelude::*;

/// Retrieves a User.
#[utoipa::path(
    get,
    path = "/users/{user_id}",
    params(
        ("user_id" = Uuid, Path),
    ),
    responses(
        (status = 200, description = "Success", body = User),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Failure"),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Extension(auth_user): Extension<User>,
    Extension(_auth_token): Extension<JsonWebTokenData>,
) -> Result<Json<User>, AppError> {
    let AppState { db } = state;

    let user = dmn::users::get_user(&db, user_id, auth_user.id).await?;
    Ok(Json(user))
}
