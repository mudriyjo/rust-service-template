mod config;
mod entity;

use axum::{http::StatusCode, routing::{get, post}, Extension, Json, Router};
use color_eyre::eyre::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, NotSet, Set};
use serde::{Deserialize, Serialize};
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::config::{Config, get_config};
use crate::entity::user;

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

    Migrator::up(&connection_pool, None).await?;


    let app = Router::new()
        .route("/", get(hello_world))
        .route("/api/users", post(add_user))
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

#[derive(Debug, Deserialize, Serialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

async fn add_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<user::Model>), StatusCode> {
    let user_model = user::ActiveModel {
        id: NotSet,
        name: Set(payload.name),
        email: Set(payload.email),
    };

    let user = user_model
        .insert(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(user)))
}