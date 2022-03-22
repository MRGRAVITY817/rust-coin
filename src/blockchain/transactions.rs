use serde::Deserialize;

use {
    crate::{
        blockchain::chain::RUSTCOIN,
        constants::BlockchainConstants::MINER_REWARD,
        result::{CoinError, Result},
        utils::hash,
        wallet::verify,
    },
    lazy_static::lazy_static,
    serde::Serialize,
    std::{
        collections::HashMap,
        sync::{Once, RwLock},
        time::SystemTime,
    },
};

pub type Amount = i64;

#[derive(Serialize, Deserialize)]
pub struct Tx {
    id: Vec<u8>,
    timestamp: SystemTime,
    tx_ins: Vec<TxIn>,
    tx_outs: Vec<TxOut>,
}

impl Tx {
    pub fn new(from: String, to: String, amount: Amount) -> Result<Self> {
        // TODO: implement it
        unimplemented!()
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
    /// Create new transaction from in/out bounds
    pub fn from_ins_and_outs(tx_ins: Vec<TxIn>, tx_outs: Vec<TxOut>) -> Self {
        let id = vec![];
        let timestamp = SystemTime::now();
        Self {
            id,
            timestamp,
            tx_ins,
            tx_outs,
        }
    }
    /// Return transaction instance with id which is a hash of its content
    pub fn hashed(self) -> Self {
        let id = hash(&self).to_vec();
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
    pub fn validate(&self) -> Result<bool> {
        let chain = RUSTCOIN.read()?;
        Ok(self
            .tx_ins
            .iter()
            .map(|tx_in| {
                chain.find_tx(tx_in.tx_id).and_then(|prev_tx| {
                    prev_tx.tx_outs.iter().nth(tx_in.index as usize).and_then(
                        |TxOut { address, .. }| {
                            if verify(tx_in.tx_id, tx_in.signature, address) {
                                return Some(true);
                            }
                            None
                        },
                    )
                })
            })
            .collect::<Option<Vec<_>>>()
            .is_some())
    }
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct TxOut {
    address: String,
    amount: Amount,
}

pub struct UTxOut {
    tx_id: String,
    index: i16,
    amount: Amount,
}

impl UTxOut {
    pub fn is_on_mempool(&self) -> Result<bool> {
        match MEMPOOL.read() {
            Ok(mempool) => {
                let exits = mempool
                    .iter()
                    .map(|(_, value)| {
                        value
                            .tx_ins
                            .iter()
                            .map(|TxIn { tx_id, index, .. }| {
                                if tx_id.eq(&self.tx_id) && index.eq(&self.index) {
                                    return Some(tx_id);
                                }
                                None
                            })
                            .collect::<Option<Vec<_>>>()
                    })
                    .collect::<Option<Vec<_>>>()
                    .is_some();
                Ok(exits)
            }
            Err(e) => Err(CoinError::CannotAccessMempool(e.to_string())),
        }
    }
}

type Mempool = RwLock<HashMap<&'static str, Tx>>;

lazy_static! {
    static ref MEMPOOL: Mempool = RwLock::new(HashMap::new());
    static ref MEM_ONCE: Once = Once::new();
}
