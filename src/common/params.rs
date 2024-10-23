use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct PaginationParams {
    pub skip: u32,
    pub limit: u32,
}
