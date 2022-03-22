use std::sync::PoisonError;

use sled::Error as SledError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, CoinError>;

#[derive(Error, Debug, Clone)]
pub enum CoinError {
    #[error("database error: {0}")]
    DatabaseError(String),
    #[error("cannot access mempool: {0}")]
    CannotAccessMempool(String),
    #[error("cannot access blockchain: {0}")]
    CannotAccessBlockchain(String),
    #[error("cannot find given block")]
    CannotFindGivenBlock,
    #[error("not enough balance (required: {required:?}, your balance: {balance:?})")]
    NotEnoughBalance { required: i64, balance: i64 },
    #[error("invalid transaction")]
    InvalidTransaction,
    #[error("Cannot read or write shared data")]
    CannotReadOrWriteSharedData,
    #[error("Error occurred while marshal/de-marshalling: {0}")]
    MarshalError(String),
}

impl From<SledError> for CoinError {
    fn from(e: SledError) -> Self {
        Self::DatabaseError(e.to_string())
    }
}

impl<T> From<PoisonError<T>> for CoinError {
    fn from(_: PoisonError<T>) -> Self {
        Self::CannotReadOrWriteSharedData
    }
}

impl From<bincode::Error> for CoinError {
    fn from(e: bincode::Error) -> Self {
        Self::MarshalError(e.to_string())
    }
}
