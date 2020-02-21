// main.rs
// Entry point - tokio

use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use log::info;

mod blog;
mod config;
mod handlers;
mod router;
mod templates;
mod types;

use config::{init_logging, OPT};
use router::router;

#[tokio::main]
async fn main() {
    init_logging(2); // For now just INFO
    let addr = format!("{}:{}", OPT.address, OPT.port)
        .parse()
        .expect("Should parse net::SocketAddr");
    let make_svc = make_service_fn(|_conn| async { Ok::<_, anyhow::Error>(service_fn(router)) });

    let server = Server::bind(&addr).serve(make_svc);

    info!("Serving deciduously-com on {}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
