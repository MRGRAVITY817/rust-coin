use super::transactions::Tx;

pub struct Blockchain;

impl Blockchain {
    /// Find transaction with given id
    pub fn find_tx(&self, tx_id: impl AsRef<str>) -> Option<Tx> {
        None
    }
}
