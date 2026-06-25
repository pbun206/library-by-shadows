use std::sync::Arc;

use anyhow::Result;
use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{delete, get, post},
};
use tokio::sync::RwLock;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    AppState,
    handlers::api::url::{delete_url, get_url, post_url},
};

/// This function serves as the entry point for running the Axum web server.
pub async fn serve(app_state: Arc<RwLock<AppState>>) -> Result<()> {
    // Create the router using the application state
    let app = create_router(app_state);

    let port = 8082_u16;

    // Bind the server to the specified address and port
    let address = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!("🚀 router initialized, now listening on port {}", port);

    // Start serving incoming connections
    axum::serve(address, app.into_make_service()).await?;

    Ok(())
}

/// This function defines the API routes for the application.
/// It takes the application state as input and sets up
/// the routes for handling different HTTP methods and endpoints.
fn create_router(app_state: Arc<RwLock<AppState>>) -> Router {
    // Setup session storage and layer
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

    // Get the current directory for serving assets
    let assets_path = std::env::current_dir().unwrap();

    // Get API routes :3
    let index_routes = Router::new()
        .route("/url", get(get_url).post(post_url).delete(delete_url))
        // .layer(cors_layer())
        ; // CORS usually API-only

    let api_routes = Router::new().nest("/index", index_routes);
    // .layer(cors_layer()) // CORS usually API-only
    // General router of our application
    Router::new()
        .nest("/api", api_routes)
        .with_state(app_state)
        // .fallback(handler_404) // Add a Fallback service for handling unknown paths
        .layer(MessagesManagerLayer)
        .layer(session_layer)
        .layer(TraceLayer::new_for_http())
}
