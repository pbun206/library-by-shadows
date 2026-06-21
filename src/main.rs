mod config;
mod db;
mod route;

use crate::config::Config;
use anyhow::Result;
use dotenv::dotenv;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::RwLock;

/// This structure represents the state of the application,
/// holding a database connection pool and app config data
pub struct AppState {
    pub pool: SqlitePool,
    pub config: Config,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load envs
    dotevoo().ok();

    // Retrieve the value of the `DATABASE_URL` from .env file
    let config = Config::init();

    // Connect to `Sqlite` database
    let pool = db::connect(&config.database_url).await?;

    // Set up the application state with the provided
    // database connection pool and app config data
    let app_state = Arc::new(RwLock::new(AppState { pool, config }));

    // Start the http server
    route::serve(app_state).await?;

    Ok(())
}
