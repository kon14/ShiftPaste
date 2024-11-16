use crate::db::types::{AccessTokenDb, RefreshTokenDb};
use crate::domain::types::{JsonWebToken, JsonWebTokenVariant};

impl From<AccessTokenDb> for JsonWebToken {
    fn from(db_res: AccessTokenDb) -> Self {
        JsonWebToken {
            user_id: db_res.user_id,
            expires_at: db_res.expires_at,
            variant: JsonWebTokenVariant::AccessToken,
        }
    }
}

impl From<RefreshTokenDb> for JsonWebToken {
    fn from(db_res: RefreshTokenDb) -> Self {
        JsonWebToken {
            user_id: db_res.user_id,
            expires_at: db_res.expires_at,
            variant: JsonWebTokenVariant::RefreshToken,
        }
    }
}
