mod config;

use axum::{routing::get, Extension, Router};
use color_eyre::eyre::Result;
use sea_orm::{Database};
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
    let connection_pool = Database::connect(&config.database_url).await?;

    let app = Router::new()
        .route("/", get(hello_world))
        .layer(Extension(connection_pool.clone()));

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
