use axum;
use anyhow::Result;
use dotenv::dotenv;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::shared::infrastructure::database;
use std::env;

mod modules;
mod shared;

fn get_host() -> String {
    env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string())
}

fn get_port() -> String {
    env::var("PORT").unwrap_or_else(|_| "8080".to_string())
}

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
    let host: String = get_host();
    let port: String = get_port();
    let address = format!("{}:{}", host, port);

    let listener = TcpListener::bind(&address).await.unwrap();
    tracing::debug!("Server: Listening on http://{}", address);

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
