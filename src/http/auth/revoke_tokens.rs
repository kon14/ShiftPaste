use axum::extract::{Path, State};
use axum::Extension;
use uuid::Uuid;

use crate::common::state::AppState;
use crate::domain as dmn;
use crate::domain::types::{JsonWebTokenData, UniqueAccessTokenIdentifier, User};
use crate::prelude::*;

/// Revokes an auth token pair.
#[utoipa::path(
    delete,
    path = "/auth/tokens/{access_token_id}",
    params(
        ("access_token_id" = Uuid, Path),
    ),
    responses(
        (status = 200, description = "Success", body = String),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Failure"),
    ),
    security(
        ("bearerAuth" = [])
    ),
)]
pub async fn revoke_tokens(
    State(state): State<AppState>,
    Path(access_token_id): Path<Uuid>,
    Extension(auth_user): Extension<User>,
    Extension(_auth_token): Extension<JsonWebTokenData>,
) -> Result<String, AppError> {
    let AppState { db } = state;

    let access_token_id = UniqueAccessTokenIdentifier::Id(access_token_id);
    dmn::auth::revoke_tokens(&db, access_token_id, auth_user.id).await?;

    Ok("Auth token pair deleted successfully.".to_string())
}
