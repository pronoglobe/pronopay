// fn main() {
//     println!("Hello, world!");
// }


use axum::{handler::get, Router};
use std::net::SocketAddr;
use tokio;

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let app = Router::new().route("/", get(handler));

    // Run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, World!"
}