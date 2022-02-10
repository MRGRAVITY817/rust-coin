mod handlers;

use {
    crate::{
        p2p::connection::upgrade,
        rest::handlers::{
            balance, block, blocks, documentation, mempool, peers, status, tx, wallet,
        },
    },
    axum::{routing::get, Router},
    std::net::SocketAddr,
};

pub async fn start(port: u16) {
    let app = Router::new()
        .route("/", get(documentation))
        .route("/status", get(status))
        .route("/block", get(blocks))
        .route("/blocks/:hash", get(block))
        .route("/balance/:address", get(balance))
        .route("/mempool", get(mempool))
        .route("/wallet", get(wallet))
        .route("/tx", get(tx))
        .route("/ws", get(upgrade))
        .route("/peers", get(peers));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("Api Server: http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
