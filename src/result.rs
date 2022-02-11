use sled::Error as SledError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, CoinError>;

#[derive(Error, Debug, Clone)]
pub enum CoinError {
    #[error("database error: {0}")]
    DatabaseError(String),
    #[error("cannot hash: {0}")]
    CannotHash(String),
}

impl From<SledError> for CoinError {
    fn from(e: SledError) -> Self {
        Self::DatabaseError(e.to_string())
    }
}
