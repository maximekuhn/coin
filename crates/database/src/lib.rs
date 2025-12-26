pub mod queries;
pub mod setup;

mod error;
mod models;
mod pagination;

pub use error::*;
pub use pagination::DbPagination;

pub use sqlx::Error as SqlxError;
pub use sqlx::SqlitePool;

pub type Transaction<'a> = sqlx::Transaction<'a, sqlx::Sqlite>;
