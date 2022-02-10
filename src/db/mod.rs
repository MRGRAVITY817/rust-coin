use {
    crate::result::Result,
    sled::{Db, IVec},
};

const DB_NAME: &'static str = "rustcoin";
const DATA_TREE: &[u8] = b"data";
const BLOCK_TREE: &[u8] = b"block";
const CHECKPOINT: &[u8] = b"checkpoint";

struct CoinDb {
    db: Db,
    name: String,
}

impl CoinDb {
    /// Create or open a Database from given name.
    pub fn from(name: impl AsRef<str>) -> Result<Self> {
        let name = format!("{DB_NAME}_{}", name.as_ref());
        let db = sled::open(name.as_str())?;
        Ok(Self { db, name })
    }
    /// Get database filename
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    /// Save a block info in `Block` tree
    pub fn save_block(&self, hash: &[u8], data: &[u8]) -> Result<()> {
        let block_tree = self.db.open_tree(BLOCK_TREE)?;
        block_tree.insert(hash, data)?;
        Ok(())
    }
    /// Delete blocks in tree
    pub fn empty_blocks(&self) -> Result<()> {
        self.db.remove(BLOCK_TREE)?;
        Ok(())
    }
    /// Save checkpoint of the data
    pub fn save_checkpoint(&self, data: &[u8]) -> Result<()> {
        let data_tree = self.db.open_tree(DATA_TREE)?;
        data_tree.insert(CHECKPOINT, data)?;
        Ok(())
    }
    /// Get checkpoint data
    pub fn get_checkpoint(&self) -> Result<Option<IVec>> {
        let data_tree = self.db.open_tree(DATA_TREE)?;
        let checkpoint = data_tree.get(CHECKPOINT)?;
        Ok(checkpoint)
    }
    /// Get block data
    pub fn get_block(&self, hash: &[u8]) -> Result<Option<IVec>> {
        let block_tree = self.db.open_tree(BLOCK_TREE)?;
        let block = block_tree.get(hash)?;
        Ok(block)
    }
}
