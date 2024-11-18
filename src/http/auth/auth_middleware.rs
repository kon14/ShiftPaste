use axum::extract::{FromRequestParts, Request, State};
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::Response;

use crate::common::state::AppState;
use crate::db;
use crate::domain::types::{JsonWebTokenData, UniqueUserIdentifier};
use crate::prelude::*;

pub async fn auth_middleware(
    State(state): State<AppState>,
    _headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    const UNAUTHORIZED_ERR_STR: &str = "Failed to authenticate user!";

    let (mut parts, body) = request.into_parts();
    let token = JsonWebTokenData::from_request_parts(&mut parts, &state)
        .await
        .map_err(|err| err.reword(UNAUTHORIZED_ERR_STR.to_string()))?;

    let user = db::users::get_user(&state.db, UniqueUserIdentifier::Id(token.user_id)).await?;

    let mut request = Request::from_parts(parts, body);
    request.extensions_mut().insert(user);
    request.extensions_mut().insert(token);

    let response = next.run(request).await;
    Ok(response)
}
