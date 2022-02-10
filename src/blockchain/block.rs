use serde::Serialize;

use super::transactions::Tx;

#[derive(Serialize)]
pub struct Block {
    hash: String,
    prev_hash: String,
    height: i64,
    difficulty: i64,
    nonce: i64,
    timestamp: i64,
    transactions: Vec<Tx>,
}
