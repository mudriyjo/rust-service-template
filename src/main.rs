mod config;

use axum::{Router, routing::get};
use color_eyre::eyre::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::config::{Config, get_config};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let fmt = tracing_subscriber::fmt::layer();
    let filter = tracing_subscriber::EnvFilter::from_default_env();
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt)
        .with(ErrorLayer::default())
        .init();

    dotenvy::dotenv().ok();

    let config: Config = get_config()?;
    let app = Router::new().route("/", get(|| hello_world()));
    let server_start_string = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(server_start_string)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn hello_world() -> String {
    "Hello, world!".to_string()
}
