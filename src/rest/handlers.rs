use axum::{extract::Path, response::IntoResponse};

pub async fn documentation() -> impl IntoResponse {
    let docs = [
        ("/", "GET", "See Documentation", ""),
        ("/blocks", "POST", "Add blocks", "data:string"),
        ("/blocks/{hash}", "GET", "See a block", ""),
        ("/status", "GET", "See the Status of the Blockchain", ""),
        ("/balance/{address}", "GET", "Get TxOuts for an Address", ""),
        ("/mempool", "GET", "Get unconfirmed txs", ""),
        ("/wallet", "GET", "Get wallet info", ""),
        (
            "/transactions",
            "POST",
            "Send transactions",
            "data:addTxPayload",
        ),
        ("/ws", "GET", "Upgrade to websockets", ""),
        ("/peers", "POST", "Add peer network", "data:addPeerPayload"),
    ];

    docs.map(|(url, method, description, payload)| {
        format!(
            "\n{: <15}: {: <32}\n{: <15}: {: <32}\n{: <15}: {: <32}\n{: <15}: {: <32}\n",
            "URL", url, "Method", method, "Description", description, "Payload", payload
        )
    })
    .join(format!("\n{:=^47}\n", "").as_str())
}

pub async fn status() -> impl IntoResponse {
    format!("This will show blockchain status")
}

pub async fn blocks() -> impl IntoResponse {
    format!("This will show blocks in blockchain")
}

pub async fn block(Path(hash): Path<String>) -> impl IntoResponse {
    format!("This will show info about block {hash}")
}

pub async fn balance(Path(address): Path<String>) -> impl IntoResponse {
    format!("This will show balance of given address: {address}")
}

pub async fn mempool() -> impl IntoResponse {
    format!("This will show the mempool info")
}

pub async fn wallet() -> impl IntoResponse {
    format!("This will show your wallet info")
}

pub async fn tx() -> impl IntoResponse {
    format!("This will show the transactions")
}

pub async fn peers() -> impl IntoResponse {
    format!("This will show the peers connected in chain")
}
