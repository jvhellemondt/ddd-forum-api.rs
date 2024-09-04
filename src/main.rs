use axum;
use anyhow::Result;
use dotenv::dotenv;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::shared::infrastructure::database;

mod modules;
mod shared;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database
    if let Err(e) = database::init::execute().await {
        tracing::error!("Database initialization error: {}", e);
        return Err(e.into());
    }

    // Webserver
    let app = shared::infrastructure::api::init::initialize_app();
    let host: &str = "127.0.0.1";
    let port: u16 = 8080;
    let address = format!("{}:{}", host, port);

    let listener = TcpListener::bind(address).await.unwrap();
    tracing::debug!("Server: Listening on http://localhost:{}", port);

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
