use anyhow::{Context, Result};
use libsqlite3_sys::sqlite3_auto_extension;
use sqlite_vec::sqlite3_vec_init;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

const MAX_CONNECTIONS: u32 = 10;

/// Create a new `SqlitePoolOptions` instance and set the
/// maximum number of connections in the connection pool to 10.
pub async fn connect(pool_url: &str) -> Result<SqlitePool> {
    unsafe {
        sqlite3_auto_extension(Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(
                *mut libsqlite3_sys::sqlite3,
                *mut *mut i8,
                *const libsqlite3_sys::sqlite3_api_routines,
            ) -> i32,
        >(sqlite3_vec_init as *const ())));
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(pool_url)
        .await
        .context("Error: 🔥 unable to connect to database :<")?;

    println!("✅ Successfully connected to database :3");

    Ok(pool)
}
