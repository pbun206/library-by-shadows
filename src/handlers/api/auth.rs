use anyhow::anyhow;
use axum::{Json, extract::State, http::StatusCode};
use jsonwebtoken::{EncodingKey, Header, encode, get_current_timestamp};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    AppState,
    dto::auth::{Claims, PostLogin, PostLoginResponse, PostRegister},
    error::AppError,
    services::accounts::{authenticate_user, create_user},
};

/// Handler for GET me for user info based on JWT token
pub async fn get_me() {}

/// Handler for POST register
pub async fn post_register(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Json(payload): Json<PostRegister>,
) -> Result<StatusCode, AppError> {
    let state = app_state.read().await;
    create_user(
        &payload.email,
        &payload.password,
        &payload.username,
        &state.pool,
        &state.config.pepper,
    )
    .await?;
    Ok(StatusCode::CREATED)
}

/// Handler for POST login, return authication stuff
// TODO add tests
pub async fn post_login(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Json(payload): Json<PostLogin>,
) -> Result<Json<PostLoginResponse>, AppError> {
    let state = app_state.read().await;
    let user = authenticate_user(
        &payload.username_or_email,
        &payload.password,
        &state.pool,
        &state.config.pepper,
    )
    .await?;

    let claims = Claims {
        subject: user.uuid,
        exp: get_current_timestamp() as usize + state.config.jwt_expires_in * 60,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_bytes()),
    )
    .map_err(|e| anyhow!("JWT error: {}", e))?;

    Ok(Json(PostLoginResponse::new(token)))
}

/// Handler for POST logout
pub async fn post_logout() {}

#[cfg(test)]
mod tests {
    use crate::handlers::testing::testing_server;
    use serde_json::json;

    use super::*;

    // My first ever axum sqlx test
    #[tokio::test]
    async fn post_register_no_error() {
        let test_server = testing_server().await;
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
    #[tokio::test]
    async fn post_register_duplicate_email_error() {
        let test_server = testing_server().await;
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
    fn post_register_duplicate_username_error() {
        let test_server = testing_server().await;
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
