use {
    super::{
        block::Block,
        transactions::{Amount, Tx},
    },
    crate::{
        db::{CoinDb, DB},
        result::Result,
        utils::to_bytes,
    },
    lazy_static::lazy_static,
    serde::{Deserialize, Serialize},
    std::sync::RwLock,
};

lazy_static! {
    pub static ref RUSTCOIN: RwLock<Blockchain> = RwLock::new(Blockchain::new());
}

#[derive(Serialize, Deserialize)]
pub struct Blockchain {
    latest_hash: String,
    height: i64,
    current_difficulty: i64,
}

impl Blockchain {
    /// Create a blockchain instance
    pub fn new() -> Self {
        Self {
            latest_hash: "RUSTCOIN".to_string(),
            height: 0,
            current_difficulty: 0,
        }
    }
    /// Return chain data
    pub fn chain(&self) -> &Self {
        self
    }
    /// Find transaction with given id
    pub fn find_tx(&self, tx_id: impl AsRef<str>) -> Option<Tx> {
        None
    }
    /// Get balance of the address owner
    pub fn balance(&self, address: String) -> Result<Amount> {
        Ok(1000)
    }
    /// Get current difficulty
    pub fn difficulty(&self) -> i64 {
        self.current_difficulty
    }
    /// Set latest hash
    pub fn set_latest_hash(&mut self, latest_hash: String) {
        self.latest_hash = latest_hash;
    }
    /// Increase block height
    pub fn increase_height(&mut self) {
        self.height += 1;
    }
    /// Set current difficulty
    pub fn set_current_difficulty(&mut self, current_difficulty: i64) {
        self.current_difficulty = current_difficulty;
    }
    fn blocks(&self) -> Vec<&Block> {
        let hash_cursor = self.latest_hash;
    }
}

/// Persist blockchain data
pub fn persist_chain() -> Result<()> {
    let chain = RUSTCOIN.read()?.chain();
    let db = DB.read()?;
    db.save_checkpoint(to_bytes(chain)?)
}

/// Add a block in chain
pub fn add_block(db: &CoinDb) -> Result<Block> {
    let chain = RUSTCOIN.read()?;
    let block = Block::new(
        chain.latest_hash.clone(),
        chain.height + 1,
        chain.difficulty(),
    );

    let mut chain = RUSTCOIN.write()?;
    chain.latest_hash = block.hash().to_string();
    chain.height = block.height();
    chain.current_difficulty = block.difficulty();
    persist_chain();
    Ok(block)
}

/// We add peer block when we get broadcasted message about new mined block
pub fn add_peer_block(new_block: &Block) -> Result<&Block> {
    let mut chain = RUSTCOIN.write()?;
    chain.increase_height();
    chain.set_latest_hash(new_block.hash().to_string());
    chain.set_current_difficulty(new_block.difficulty());
    persist_chain()?;
    Ok(new_block)
}
