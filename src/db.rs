use {
    crate::{
        constants::DatabaseConstants::{BLOCK_TREE, CHECKPOINT, DATA_TREE, DB_NAME},
        result::Result,
    },
    lazy_static::lazy_static,
    sled::{Db, IVec},
    std::sync::RwLock,
};

lazy_static! {
    pub static ref DB: RwLock<CoinDb> = RwLock::new(CoinDb::from("4000"));
}

pub struct CoinDb {
    db: Result<Db>,
    name: String,
}

impl CoinDb {
    /// Create or open a Database from given name.
    pub fn from(name: impl AsRef<str>) -> Self {
        let open_db = |path| -> Result<Db> {
            let db = sled::open(path)?;
            Ok(db)
        };
        let name = format!("{DB_NAME}_{}", name.as_ref());
        let db = open_db(name);
        Self { db, name }
    }
    /// Get database
    pub fn db(&self) -> Result<Db> {
        self.db
    }
    /// Get database filename
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    /// Save a block info in `Block` tree
    pub fn save_block(&self, hash: &[u8], data: &[u8]) -> Result<()> {
        let block_tree = self.db?.open_tree(BLOCK_TREE)?;
        block_tree.insert(hash, data)?;
        Ok(())
    }
    /// Delete blocks in tree
    pub fn empty_blocks(&self) -> Result<()> {
        self.db?.remove(BLOCK_TREE)?;
        Ok(())
    }
    /// Save checkpoint of the data
    pub fn save_checkpoint(&self, data: &[u8]) -> Result<()> {
        let data_tree = self.db?.open_tree(DATA_TREE)?;
        data_tree.insert(CHECKPOINT, data)?;
        Ok(())
    }
    /// Get checkpoint data
    pub fn get_checkpoint(&self) -> Result<Option<IVec>> {
        let data_tree = self.db?.open_tree(DATA_TREE)?;
        let checkpoint = data_tree.get(CHECKPOINT)?;
        Ok(checkpoint)
    }
    /// Get block data
    pub fn get_block(&self, hash: &[u8]) -> Result<Option<IVec>> {
        let block_tree = self.db?.open_tree(BLOCK_TREE)?;
        let block = block_tree.get(hash)?;
        Ok(block)
    }
}
