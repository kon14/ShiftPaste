use axum::extract::{Path, State};
use axum::Extension;
use uuid::Uuid;

use crate::common::state::AppState;
use crate::domain as dmn;
use crate::domain::types::{JsonWebTokenData, User};
use crate::prelude::*;

/// Deletes a User.
#[utoipa::path(
    delete,
    path = "/users/{user_id}",
    params(
        ("user_id" = Uuid, Path),
    ),
    responses(
        (status = 200, description = "Success", body = String),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Failure"),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Extension(auth_user): Extension<User>,
    Extension(_auth_token): Extension<JsonWebTokenData>,
) -> Result<String, AppError> {
    let AppState { db } = state;

    dmn::users::delete_user(&db, user_id, auth_user.id).await?;
    Ok(format!("User ({}) deleted successfully.", user_id))
}
