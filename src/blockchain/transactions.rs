use serde::Serialize;

#[derive(Serialize)]
pub struct Tx {
    id: String,
    timestamp: i32,
    tx_ins: Vec<TxIn>,
    tx_outs: Vec<TxOut>,
}

#[derive(Serialize)]
pub struct TxIn {
    tx_id: String,
    index: i64,
    signature: String,
}

#[derive(Serialize)]
pub struct TxOut {
    address: String,
    amount: i32,
}
