pub mod DatabaseConstants {
    pub const DB_NAME: &'static str = "rustcoin";
    pub const DATA_TREE: &[u8] = b"data";
    pub const BLOCK_TREE: &[u8] = b"block";
    pub const CHECKPOINT: &[u8] = b"checkpoint";
}

pub mod BlockchainConstants {
    pub const DEFAULT_DIFFICULTY: i64 = 2;
    pub const DIFFICULTY_INTERVAL: i64 = 5;
    pub const BLOCK_INTERVAL: i64 = 2;
    pub const ALLOWED_RANGE: i64 = 2;
    pub const MINER_REWARD: i64 = 50;
}
