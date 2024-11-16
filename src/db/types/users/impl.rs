use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::postgres::PgValueRef;
use sqlx::{
    postgres::{PgTypeInfo, Postgres},
    Database, Decode, Encode, Type,
};
use std::error::Error;

use crate::db::types::UserDb;
use crate::domain::types::{Email, User};
use crate::prelude::*;

impl Type<Postgres> for Email {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("email")
    }
}

impl<'a> Encode<'a, Postgres> for Email {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as Database>::ArgumentBuffer<'a>,
    ) -> Result<IsNull, BoxDynError> {
        for &byte in self.0.as_bytes() {
            buf.push(byte);
        }
        Ok(IsNull::No)
    }
}

impl<'a> Decode<'a, Postgres> for Email {
    fn decode(value: PgValueRef<'a>) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let email_str: String = Decode::<Postgres>::decode(value)?;
        let email = Email::try_from_db(&email_str)?;
        Ok(email)
    }
}

impl TryFrom<UserDb> for User {
    type Error = AppError;

    fn try_from(db_res: UserDb) -> Result<Self, Self::Error> {
        Ok(User {
            id: db_res.id,
            email: Email::try_from_db(&db_res.email)?,
            created_at: db_res.created_at,
            updated_at: db_res.updated_at,
        })
    }
}
