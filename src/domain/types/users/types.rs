use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub email: Email,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Email(pub(crate) String);

pub enum UniqueUserIdentifier {
    Id(Uuid),
    Email(Email),
}
