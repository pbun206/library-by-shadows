use axum_test::TestServer;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{AppState, route::create_router, testing::testing_pool};

pub async fn testing_server() -> TestServer {
    let pool = testing_pool().await;
    let app = Arc::new(RwLock::new(AppState::with_test_config(pool)));
    TestServer::new(create_router(app))
}
