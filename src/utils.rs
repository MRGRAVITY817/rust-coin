use std::borrow::Borrow;

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::{blockchain::block::Block, result::Result};

pub fn hash<'a>(data: impl Serialize) -> &'a [u8] {
    let ser_data = bincode::serialize(&data).unwrap_or(vec![]);
    let mut hasher = Sha256::new();
    hasher.update(ser_data);
    hasher.finalize().to_vec().as_slice()
}

pub fn to_bytes<'a>(data: impl Serialize) -> Result<&'a [u8]> {
    let result = bincode::serialize(&data).map(|res| res.as_slice())?;
    Ok(result)
}

pub fn from_bytes<'a>(block: &'a Block, data: &'a [u8]) -> Result<Block> {
    let result = bincode::deserialize::<Block>(data)?;
    Ok(result)
}
