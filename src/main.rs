use std::net::{IpAddr, SocketAddr};
use axum::Router;
use axum::routing::get;

use account_manager::key;

#[tokio::main]
pub async fn main() {
    let app = Router::new()
        .route("/:index", get(key));

    let addr = SocketAddr::new(IpAddr::from([127, 0, 0, 1]), 3000);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}