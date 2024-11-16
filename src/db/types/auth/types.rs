use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct AccessTokenDb {
    pub id: Uuid,
    pub user_id: Uuid,
    pub jwt: String,
    pub expires_at: DateTime<Utc>,
}

pub struct RefreshTokenDb {
    pub id: Uuid,
    pub user_id: Uuid,
    pub access_token_id: Uuid,
    pub jwt: String,
    pub expires_at: DateTime<Utc>,
}
