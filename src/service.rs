use hyper::{header::CONTENT_TYPE, Body, Method, Request, Response, StatusCode};
use prometheus::{Encoder, IntGauge, GaugeVec, IntGaugeVec, TextEncoder};
use std::collections::HashMap;
use web3::types::{BlockNumber, BlockId, SyncState, TxpoolStatus};
use std::convert::TryInto;


lazy_static! {
    static ref GETH_VERSION: IntGaugeVec = register_int_gauge_vec!(
        "geth_version",
        "Client version",
        &["value"]
    )
    .unwrap();

    static ref GETH_NETWORK: IntGaugeVec = register_int_gauge_vec!(
        "geth_network",
        "Client network",
        &["value"]
    )
    .unwrap();

    static ref GETH_LATEST: IntGaugeVec = register_int_gauge_vec!(
        "geth_latest",
        "Latest block information",
        &["hash"]
    )
    .unwrap();

    static ref BITCOIND_BLOCKCHAIN_SYNC: GaugeVec = register_gauge_vec!(
        "bitcoind_blockchain_sync",
        "Blockchain sync info",
        &["type"]
    )
    .unwrap();

    static ref GETH_GAS_PRICE: IntGauge = register_int_gauge!(
        "geth_gas_price",
        "Current gas price in wei"
    )
    .unwrap();

    static ref GETH_MEMPOOL_SIZE: IntGaugeVec = register_int_gauge_vec!(
        "geth_mempool_size",
        "Mempool information",
        &["type"]
    )
    .unwrap();

    static ref GETH_PEERS: IntGaugeVec = register_int_gauge_vec!(
        "geth_peers",
        "Connected peers",
        &["version"]
    )
    .unwrap();

    static ref CLIENT_NETWORK_NAME: HashMap<i32, &'static str> = [
        (1, "mainnet"),
        (2, "morden"),
        (3, "ropsten"),
        (4, "rinkeby"),
        (5, "goerli"),
        (6, "kotti"),
        (7, "mordor"),
        (42, "kovan"),
        (2018, "dev"),
     ].iter().cloned().collect();
}

pub async fn update_metrics(node: &String) -> Result<(), web3::Error> {
    let transport = web3::transports::Http::new(node)?;
    let web3 = web3::Web3::new(transport);

    let client_version = web3.web3().client_version().await?;
    GETH_VERSION.with_label_values(&[&client_version]).set(1);

    let client_network: i32 = web3.net().version().await?.parse().unwrap();
    GETH_NETWORK.with_label_values(&[&CLIENT_NETWORK_NAME[&client_network]]).set(1);

    let latest_block = web3.eth().block(BlockId::Number(BlockNumber::Latest)).await?.unwrap();
    let latest_hash = format!("{:#x}", latest_block.hash.unwrap());
    let latest_number = latest_block.number.unwrap().as_u64().try_into().unwrap();
    if GETH_LATEST.with_label_values(&[&latest_hash]).get() != latest_number {
        GETH_LATEST.reset();
        GETH_LATEST
            .with_label_values(&[&latest_hash])
            .set(latest_number);

        info!("update latest to {} - {}", latest_number, latest_hash);

        let (current, highest, progress) = match web3.eth().syncing().await? {
            SyncState::Syncing(sync_info) => {
                let current: u64 = sync_info.current_block.as_u64().try_into().unwrap();
                let current: f64 = current as f64;
                let highest: u64 = sync_info.highest_block.as_u64().try_into().unwrap();
                let highest: f64 = highest as f64;
                let progress = current / highest;
                (current, highest, progress)
            }
            SyncState::NotSyncing => (latest_number as f64, latest_number as f64, 1.0),
        };
        BITCOIND_BLOCKCHAIN_SYNC.with_label_values(&["current"]).set(current);
        BITCOIND_BLOCKCHAIN_SYNC.with_label_values(&["highest"]).set(highest);
        BITCOIND_BLOCKCHAIN_SYNC.with_label_values(&["progress"]).set(progress);
    }


    let gas_price = web3.eth().gas_price().await?;
    GETH_GAS_PRICE.set(gas_price.as_u64().try_into().unwrap());

    let mempool: TxpoolStatus = web3.txpool().status().await?;
    GETH_MEMPOOL_SIZE.with_label_values(&["size"]).set(mempool.queued.as_u64().try_into().unwrap());

    let peers = web3.net().peer_count().await?;
    GETH_PEERS.with_label_values(&["all"]).set(peers.as_u64().try_into().unwrap());

    Ok(())
}

pub async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/metrics") => {
            let encoder = TextEncoder::new();

            let metric_families = prometheus::gather();
            let mut buffer = vec![];
            encoder.encode(&metric_families, &mut buffer).unwrap();

            let response = Response::builder()
                .status(200)
                .header(CONTENT_TYPE, encoder.format_type())
                .body(Body::from(buffer))
                .unwrap();

            Ok(response)
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not Found".into())
            .unwrap()),
    }
}
