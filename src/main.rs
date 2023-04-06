use std::net::{IpAddr, SocketAddr};
use axum::Router;
use axum::routing::get;

use account_manager::key;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    production: bool,

    #[clap(long, short, default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
pub async fn main() {
    let args = Args::parse();

    let ip = match args.production {
        false => [127, 0, 0, 1],
        true => [0, 0, 0, 0],
    };

    let app = Router::new()
        .route("/:index", get(key));

    let addr = SocketAddr::new(IpAddr::from(ip), args.port);

    // Print service address and port
    let tmp = ip.map(|e| e.to_string() + ".").into_iter().collect::<String>();
    let ip_string = &tmp[0..tmp.len() - 1];
    println!("Running on {:}:{:}", ip_string, args.port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
