use sqlx::SqlitePool;

use crate::db::connect;

/// A pool for ~~swimming~~ testing.
pub async fn testing_pool() -> SqlitePool {
    let pool = connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
    pool
}
