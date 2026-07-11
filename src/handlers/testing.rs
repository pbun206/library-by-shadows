use std::sync::Arc;

use axum::Router;
use axum_test::TestServer;
use sqlx::SqlitePool;
use tokio::sync::RwLock;

use crate::{AppState, route::create_router};

pub fn testing_server(pool: SqlitePool) -> TestServer {
        let app = Arc::new(RwLock::new(AppState::with_test_config(pool)));
        TestServer::new(create_router(app))
}
