pub mod types;

pub mod auth;
pub mod snippets;
pub mod users;

use sqlx::{PgPool, Postgres, Transaction};

pub enum DbExecutor<'a, 'b> {
    Pool(&'a PgPool),
    Transaction(&'a mut Transaction<'b, Postgres>),
}
