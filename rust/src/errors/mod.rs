use std::error::Error as StdError;
use std::result::Result as StdResult;

/// A specialized `Result` type for SQLx.
pub type Result<T> = StdResult<T, Error>;

// Convenience type alias for usage within SQLx.
// Do not make this type public.
pub type BoxDynError = Box<dyn StdError + 'static + Send + Sync>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("sqlx connect error: {0}")]
    DBConnectError(#[source] sqlx::Error),
    #[error("sqlx query error: {0}")]
    DBQueryError(#[source] sqlx::Error),
    #[error("error with configuration: {0}")]
    DBMigrate(#[source] sqlx::migrate::MigrateError),
    // #[error("error with configuration: {0}")]
    // Database(#[source] sqlx::Error),
    #[error("invalid params: {0}")]
    InvalidParams(String),
    #[error("serialize error: {0}")]
    Serialize(String),
    #[error("tokio runtime error: {0}")]
    Runtime(String),
    #[error("page query params error: {0}")]
    InvalidPageableError(String),
}
