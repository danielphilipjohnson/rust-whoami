mod handlers;
mod models;
mod utils;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

use crate::handlers::whoami::handle_whoami;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/whoami", get(handle_whoami))
        .layer(TraceLayer::new_for_http())
        .into_make_service_with_connect_info::<SocketAddr>();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
