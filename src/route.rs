use anyhow::Result;
use axum::{
    Router,
    // middleware::from_fn_with_state,
    routing::{get, post},
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing::info;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    AppState,
    handlers::{
        api::{
            auth::{get_me, post_login, post_register},
            urls::{delete_url, get_url, post_url},
        },
        web::{
            info::get_about_page,
            search::{get_search_page, home},
        },
    },
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
pub fn create_router(app_state: Arc<RwLock<AppState>>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods(Any)
        // TODO
        .allow_origin(Any);

    // Setup session storage and layer
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

    let public_path = std::env::current_dir().unwrap().join("public");

    // Get API routes :3
    let index_routes = Router::new().route("/urls", get(get_url).post(post_url).delete(delete_url));

    let auth_routes = Router::new()
        .route("/register", post(post_register))
        .route("/me", get(get_me))
        .route("/login", post(post_login));

    // Cors for API routes only
    let api_routes = Router::new()
        .nest("/index", index_routes)
        .nest("/auth", auth_routes)
        .layer(cors);

    // Cors for API routes only
    let web_routes = Router::new()
        .route("/search", get(get_search_page))
        .route("/", get(home))
        .route("/about", get(get_about_page));

    // General router of our application
    Router::new()
        .nest("/api", api_routes)
        .merge(web_routes)
        .nest_service(
            "/public",
            ServeDir::new(public_path.to_str().unwrap()), // Serve static assets
        )
        .with_state(app_state)
        // .fallback(handler_404) // Add a Fallback service for handling unknown paths
        // .layer(MessagesManagerLayer)
        .layer(session_layer)
    // .layer(TraceLayer::new_for_http())
}
