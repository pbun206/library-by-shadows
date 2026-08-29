mod config;
mod db;
mod dto;
mod error;
mod handlers;
mod model;
mod route;
mod services;
#[cfg(test)]
mod testing;

use crate::{config::Config, services::vector_embeding::Embeder};
use anyhow::Result;
use dotenv::dotenv;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::RwLock;

use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// This structure represents the state of the application,
/// holding a database connection pool and app config data
pub struct AppState {
    pub pool: SqlitePool,
    pub embeder: Embeder,
    pub config: Config,
}

impl AppState {
    pub fn new(pool: SqlitePool, config: Config) -> Self {
        Self {
            pool,
            embeder: Embeder::try_new().unwrap(),
            config,
        }
    }

    pub fn with_test_config(pool: SqlitePool) -> Self {
        Self {
            pool,
            embeder: Embeder::try_new().unwrap(),
            config: Config::test_default(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load envs
    dotenv().ok();

    // Get tracing working
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "lbs=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing config…");
    // Retrieve the value of the `DATABASE_URL` from .env file
    let config = Config::init_from_env();

    info!("connecting db…");

    // Connect to `Sqlite` database
    let pool = db::connect(&config.database_url).await?;

    info!("initializing state…");
    // Set up the application state with the provided
    // database connection pool and app config data
    let app_state = Arc::new(RwLock::new(AppState::new(pool, config)));

    info!("initializing router…");
    // Start the http server
    route::serve(app_state).await?;

    Ok(())
}
