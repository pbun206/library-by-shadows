use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{Json, extract::State, http::StatusCode};

use crate::{AppState, dto::auth::PostRegister, error::AppError, services::accounts::create_user};

/// Handler for GET me for user info based on JWT token
pub async fn get_me() {}

/// Handler for POST register
pub async fn post_register(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Json(payload): Json<PostRegister>,
) -> Result<StatusCode, AppError> {
    let state = app_state.read().await;
    create_user(
        payload.email,
        payload.password,
        payload.username,
        &state.pool,
    )
    .await?;
    Ok(StatusCode::CREATED)
}

/// Handler for POST login
pub async fn post_login() {}

/// Handler for POST logout
pub async fn post_logout() {}

#[cfg(test)]
mod tests {
    use crate::handlers::testing::testing_server;
    use serde_json::json;
    use sqlx::SqlitePool;

    use super::*;

    // My first ever axum sqlx test
    #[sqlx::test]
    fn post_register_no_error(pool: SqlitePool) {
        let test_server = testing_server(pool);
        let response = test_server
            .post("/api/auth/register")
            .json(&json!(PostRegister {
                username: "thairanaru".to_string(),
                email: "thairanaru@proton.me".to_string(),
                password: "!testpassword123!".to_string(),
            }))
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
    }

    // Ensure duplicate emails are handled :3
    #[sqlx::test]
    fn post_register_duplicate_email_error(pool: SqlitePool) {
        let test_server = testing_server(pool);
        let response = test_server
            .post("/api/auth/register")
            .json(&json!(PostRegister {
                username: "thairanaru".to_string(),
                email: "thairanaru@proton.me".to_string(),
                password: "!testpassword123!".to_string(),
            }))
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);

        let response = test_server
            .post("/api/auth/register")
            .json(&json!(PostRegister {
                username: "naruthaira".to_string(),
                email: "thairanaru@proton.me".to_string(),
                password: "!testpassword123!".to_string(),
            }))
            .await;

        assert_eq!(response.status_code(), StatusCode::CONFLICT);
    }

    // Ensure duplicate emails are handled :3
    #[sqlx::test]
    fn post_register_duplicate_username_error(pool: SqlitePool) {
        let test_server = testing_server(pool);
        let response = test_server
            .post("/api/auth/register")
            .json(&json!(PostRegister {
                username: "thairanaru".to_string(),
                email: "thairanaru@proton.me".to_string(),
                password: "!testpassword123!".to_string(),
            }))
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);

        let response = test_server
            .post("/api/auth/register")
            .json(&json!(PostRegister {
                username: "thairanaru".to_string(),
                email: "naruthaira@proton.me".to_string(),
                password: "!testpassword123!".to_string(),
            }))
            .await;

        assert_eq!(response.status_code(), StatusCode::CONFLICT);
    }
}
