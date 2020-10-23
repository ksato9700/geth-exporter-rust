use hyper::{header::CONTENT_TYPE, Body, Method, Request, Response, StatusCode};
use prometheus::{Encoder, GaugeVec, IntGaugeVec, TextEncoder};
use std::collections::HashMap;

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

    static ref BITCOIND_BLOCKCHAIN_SYNC: IntGaugeVec = register_int_gauge_vec!(
        "bitcoind_blockchain_sync",
        "Blockchain sync info",
        &["type"]
    )
    .unwrap();

    static ref MG: GaugeVec = register_gauge_vec!(
        "test_macro_gauge_vec_3",
         "help",
         &["a", "b"]
    ).unwrap();

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


    // static ref HTTP_COUNTER: Counter = register_counter!(opts!(
    //     "example_http_requests_total",
    //     "Total number of HTTP requests made.",
    //     labels! {"handler" => "all",}
    // ))
    // .unwrap();
    // static ref HTTP_BODY_GAUGE: Gauge = register_gauge!(opts!(
    //     "example_http_response_size_bytes",
    //     "The HTTP response sizes in bytes.",
    //     labels! {"handler" => "all",}
    // ))
    // .unwrap();
    // // static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
    //     "example_http_request_duration_seconds",
    //     "The HTTP request latencies in seconds.",
    //     &["handler"]
    // )
    // .unwrap();
}

pub async fn update_metrics(node: &String) -> Result<(), web3::Error> {
    let transport = web3::transports::Http::new(node)?;
    let web3 = web3::Web3::new(transport);

    let client_version = web3.web3().client_version().await?;
    GETH_VERSION.with_label_values(&[&client_version]).set(1);
    // println!("client_version: {}", client_version);

    let client_network = web3.net().version().await?;
    let client_network: i32 = client_network.parse().unwrap();
    GETH_NETWORK
        .with_label_values(&[&CLIENT_NETWORK_NAME[&client_network]])
        .set(1);
    // println!("client_network: {}", client_network);

    let latest_block = web3.eth().block_number().await?;
    // println!("latest_block: {}", latest_block);

    let sync_info = web3.eth().syncing().await?;
    // println!("sync_info: {:?}", sync_info);

    let gas_price = web3.eth().gas_price().await?;
    // println!("gas_price: {:?}", gas_price);

    let mempool = web3.txpool().status().await?;
    // println!("mempool: {:?}", mempool);

    Ok(())
}
// async fn f() {
//     let mut interval = time::interval(time::Duration::from_millis(100));
//     loop {
//         interval.tick().await;
//         update_metrics();
//         println!("metrics updated");
//     }
// }

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

            // timer.observe_duration();

            Ok(response)
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not Found".into())
            .unwrap()),
    }
}
