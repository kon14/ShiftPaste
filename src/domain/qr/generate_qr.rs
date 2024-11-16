use qrcode::render::svg;
use qrcode::QrCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::db;
use crate::prelude::*;

pub async fn generate_qr(db: &PgPool, snippet_id: Uuid) -> Result<String, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to generate QR code for snippet!";
    const API_REDIRECT_PATH: &str = "/redirect";
    const QR_MIN_PIXELS: u32 = 200;
    const QR_FG_COLOR: &str = "#000000";
    const QR_BG_COLOR: &str = "#FFFFFF";

    let _ = db::snippets::get_snippet(db, snippet_id, Some(false)).await?;

    let api_base_url = crate::common::utils::get_api_base_url();
    let snippet_redirect_url = format!("{api_base_url}{API_REDIRECT_PATH}/{snippet_id}");

    let qr = QrCode::new(snippet_redirect_url)
        .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;
    let svg = qr
        .render()
        .min_dimensions(QR_MIN_PIXELS, QR_MIN_PIXELS)
        .dark_color(svg::Color(QR_FG_COLOR))
        .light_color(svg::Color(QR_BG_COLOR))
        .build();

    Ok(svg)
}
