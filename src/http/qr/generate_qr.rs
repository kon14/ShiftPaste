use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use base64::engine::general_purpose;
use base64::Engine;
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::common::state::AppState;
use crate::domain as dmn;
use crate::prelude::*;

#[derive(Deserialize, IntoParams)]
pub struct GenerateQrCodeHttpQueryParams {
    /// Whether to return the QR code as a downloadable file instead of a Base64-encoded one.
    // https://github.com/tafia/quick-xml/issues/497#issuecomment-1686470888
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub download: Option<bool>,
}

/// Generates a URL-based QR code pointing to the Snippet's redirection page.
#[utoipa::path(
    post,
    path = "/qr/{snippet_id}",
    params(
        ("snippet_id" = Uuid, Path),
        GenerateQrCodeHttpQueryParams,
    ),
    responses(
        (status = 200, description = "Success", body = String),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Failure"),
    ),
)]
pub async fn generate_qr(
    State(state): State<AppState>,
    Path(snippet_id): Path<Uuid>,
    Query(query): Query<GenerateQrCodeHttpQueryParams>,
) -> Result<Response, AppError> {
    const FILENAME_PREFIX: &str = "shiftpaste_snippet_";

    let AppState { db } = state;

    let qr_img = dmn::qr::generate_qr(&db, &snippet_id).await?;

    let content_type = "image/svg+xml";
    if let Some(true) = query.download {
        let file_name = format!("{FILENAME_PREFIX}{snippet_id}");
        let file_type = "svg";

        let response = Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", content_type)
            .header(
                "Content-Disposition",
                format!("attachment; filename=\"{file_name}.{file_type}\""),
            )
            .body(qr_img.as_bytes().to_vec().into())
            .map_err(|err| {
                AppError::internal_with_private("Failed to create response!", err.to_string())
            })?;
        Ok(response)
    } else {
        let qr_base64 = general_purpose::STANDARD.encode(qr_img.as_bytes());
        let qr_base64 = format!("data:{content_type};base64,{qr_base64}");
        Ok(qr_base64.into_response())
    }
}
