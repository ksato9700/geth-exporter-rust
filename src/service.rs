// use prometheus::{Counter, Encoder, Gauge, HistogramVec, TextEncoder};
use prometheus::{Encoder, IntGaugeVec, GaugeVec, TextEncoder};
use tokio::time;
use hyper::{
    header::CONTENT_TYPE,
    Body, Request, Response, Method, StatusCode
};

// #[tokio::main]
// async fn main() -> web3::Result<()> {
//     let args: Vec<String> = env::args().collect();
//     let transport = web3::transports::Http::new(&args[1])?;
//     let web3 = web3::Web3::new(transport);

//     let client_version = web3.web3().client_version().await?;
//     println!("client_version: {}", client_version);

//     let client_network = web3.net().version().await?;
//     println!("client_network: {}", client_network);

//     let latest_block = web3.eth().block_number().await?;
//     println!("latest_block: {}", latest_block);

//     let sync_info = web3.eth().syncing().await?;
//     println!("sync_info: {:?}", sync_info);

//     let gas_price = web3.eth().gas_price().await?;
//     println!("gas_price: {:?}", gas_price);

//     let mempool = web3.txpool().status().await?;
//     println!("mempool: {:?}", mempool);

//     Ok(())
// }


lazy_static! {
    static ref GETH_VERSION: IntGaugeVec = register_int_gauge_vec!(
        "geth_version",
        "Client version",
        &["value"]
    )
    .unwrap();


    static ref MG: GaugeVec = register_gauge_vec!(
        "test_macro_gauge_vec_3",
         "help",
         &["a", "b"]
    ).unwrap();


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


pub async fn update_metrics(interval: u64) {
    let mut interval = time::interval(time::Duration::from_millis(interval));
    println!("before: {:?}", interval);
    interval.tick().await;
    println!("after");

    GETH_VERSION.with_label_values(&["vvvvvvvvvvvvvvvversion"]).set(1);
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
        },
        _ => Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("Not Found".into())
        .unwrap()),

    }
}
