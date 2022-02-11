use std::time::SystemTime;

use serde::Serialize;

use crate::{utils::hash, wallet::verify};

use super::chain::Blockchain;

const MINER_REWARD: i64 = 50;

#[derive(Serialize)]
pub struct Tx {
    id: Vec<u8>,
    timestamp: SystemTime,
    tx_ins: Vec<TxIn>,
    tx_outs: Vec<TxOut>,
}

impl Tx {
    /// Create new transaction from in/out bounds
    pub fn new(tx_ins: Vec<TxIn>, tx_outs: Vec<TxOut>) -> Self {
        let id = vec![];
        let timestamp = SystemTime::now();
        Self {
            id,
            timestamp,
            tx_ins,
            tx_outs,
        }
    }
    /// Create new coinbase tx for given address
    pub fn new_coinbase(address: String) -> Self {
        let tx_ins = vec![TxIn {
            tx_id: "".to_string(),
            index: -1,
            signature: "COINBASE".to_string(),
        }];
        let tx_outs = vec![TxOut {
            address: "".to_string(),
            amount: MINER_REWARD,
        }];
        let temp_tx = Self {
            id: vec![],
            timestamp: SystemTime::now(),
            tx_ins,
            tx_outs,
        };
        temp_tx.hashed()
    }
    /// Return transaction instance with id which is a hash of its content
    pub fn hashed(self) -> Self {
        let id = hash(&self);
        Self { id, ..self }
    }
    /// Sign transaction signature created with inbounds id and private key
    pub fn sign(self) -> Self {
        let tx_ins = self
            .tx_ins
            .into_iter()
            .map(|tx_in| tx_in.set_signature("".to_string()))
            .collect();
        Self { tx_ins, ..self }
    }
    /// Validate transaction
    pub fn validate(&self, chain: &Blockchain) -> bool {
        self.tx_ins
            .iter()
            .map(
                |TxIn {
                     tx_id,
                     index,
                     signature,
                 }| {
                    chain.find_tx(tx_id).and_then(|prev_tx| {
                        prev_tx.tx_outs.iter().nth(*index as usize).and_then(
                            |TxOut { address, .. }| {
                                if verify(tx_id, signature, address) {
                                    return Some(true);
                                }
                                None
                            },
                        )
                    })
                },
            )
            .collect::<Option<Vec<_>>>()
            .is_some()
    }
}

#[derive(Serialize)]
pub struct TxIn {
    tx_id: String,
    index: i16,
    signature: String,
}

impl TxIn {
    // Set inbound transaction's signature
    pub fn set_signature(self, signature: String) -> Self {
        Self { signature, ..self }
    }
}

#[derive(Serialize)]
pub struct TxOut {
    address: String,
    amount: i64,
}

pub struct UTxOut {
    tx_id: String,
    index: i32,
    amount: i64,
}
