use anyhow::{Context, Result};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

const MAX_CONNECTIONS: u32 = 10;

/// Create a new `SqlitePoolOptions` instance and set the
/// maximum number of connections in the connection pool to 10.
pub async fn connect(pool_url: &str) -> Result<SqlitePool> {

    let pool = SqlitePoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(pool_url)
        .await
        .context("Error: 🔥 unable to connect to database!")?;

    println!("✅ Successfully connected to database!");

    Ok(pool)
}
