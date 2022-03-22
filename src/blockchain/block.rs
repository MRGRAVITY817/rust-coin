use std::borrow::Borrow;

use serde::Deserialize;
use sled::IVec;

use crate::{
    db::DB,
    result::{CoinError, Result},
    utils::from_bytes,
};

use {super::transactions::Tx, serde::Serialize, std::time::SystemTime};

#[derive(Serialize, Deserialize)]
pub struct Block {
    hash: String,
    prev_hash: String,
    height: i64,
    difficulty: i64,
    nonce: i64,
    timestamp: SystemTime,
    transactions: Vec<Tx>,
}

impl Block {
    /// Create a new block from given info
    pub fn new(prev_hash: String, height: i64, difficulty: i64) -> Self {
        Self {
            hash: "".to_string(),
            prev_hash,
            height,
            difficulty,
            nonce: 0,
            timestamp: SystemTime::now(),
            transactions: vec![],
        }
    }
    pub fn hash(&self) -> &str {
        self.hash.as_str()
    }
    pub fn height(&self) -> i64 {
        self.height
    }
    pub fn difficulty(&self) -> i64 {
        self.difficulty
    }
    /// Restore encoded(marshalled) info into Block data
    pub fn restore_from_encoded(bytes: IVec) -> Result<Block> {
        let template = Self::new("".to_string(), 0, 0);
        from_bytes(&template, bytes.borrow())
    }
}
/// Find a block by restoring marshalled info
pub fn find_block(hash: String) -> Result<Block> {
    let read_value = DB.read()?.get_block(hash.as_bytes())?;
    match read_value {
        Some(encoded) => Block::restore_from_encoded(encoded),
        None => Err(CoinError::CannotFindGivenBlock),
    }
}
