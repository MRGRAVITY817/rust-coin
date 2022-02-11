use serde::Serialize;
use sha2::{Digest, Sha256};

pub fn hash(data: impl Serialize) -> Vec<u8> {
    let data = bincode::serialize(&data).unwrap_or(vec![]);
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
