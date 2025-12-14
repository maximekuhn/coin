#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("database corrupted data: {msg}")]
    CorruptedData { msg: String },
}
