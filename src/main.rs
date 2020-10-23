use serde::Deserialize;
use std::process;
use std::net::ToSocketAddrs;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;

mod service;

fn default_interval() -> u64 {
    100
}

fn default_listen() -> String {
    "localhost:8000".to_string()
}

fn default_node() -> String {
    "http://localhost:8545/".to_string()
}

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default="default_interval")]
    interval: u64,

    #[serde(default="default_listen")]
    listen: String,

    #[serde(default="default_node")]
    node: String,
}


#[tokio::main]
async fn main() {
    let config = match envy::prefixed("GETH_EXPORTER_").from_env::<Config>(){
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    println!("{:#?}", config);
    let interval = config.interval;
    tokio::spawn(async move {
        loop {
            println!("metrics updated");
            service::update_metrics(interval).await;
        }
    });

    let addr = match config.listen.to_socket_addrs() {
        Ok(mut addrs) => addrs.next().unwrap(),
        Err(err) => {
            println!("ERROR: GET_EXPORTER_LISTEN: \"{}\": {}", config.listen, err);
            process::exit(1)
        }
    };
    println!("Listening on http://{:?}", addr);

    let serve_future = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(service::serve_req))
    }));

    if let Err(err) = serve_future.await {
        eprintln!("server error: {}", err);
    }
}
