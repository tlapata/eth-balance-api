mod api;
mod blockchain;

use axum::{Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {

    // Composing routes from api module
    let app = Router::new().merge(api::routes());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
