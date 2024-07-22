use axum;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod modules;
mod shared;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer()).init();

    let app = shared::infrastructure::api::init::initialize_app();

    // Webserver
    let host: &str = "127.0.0.1";
    let port: u16 = 8080;
    let address = format!("{}:{}", host, port);

    let listener = TcpListener::bind(address).await.unwrap();
    tracing::debug!("Server: Listening on http://localhost:{}", port);

    axum::serve(listener, app).await.unwrap();
}
