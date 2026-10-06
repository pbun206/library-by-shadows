use anyhow::anyhow;
use axum::{Json, extract::State, http::StatusCode};
use jsonwebtoken::{EncodingKey, Header, encode, get_current_timestamp};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    AppState,
    dto::auth::{Claims, GetMeResponse, PostLogin, PostLoginResponse, PostRegister},
    error::AppError,
    services::accounts::{authenticate_user, create_user, get_user_from_uuid},
};

/// Handler for GET me for user info based on JWT token :3
pub async fn get_me(
    State(app_state): State<Arc<RwLock<AppState>>>,
    claims: Claims,
) -> Result<Json<GetMeResponse>, AppError> {
    let state = app_state.read().await;
    let user = get_user_from_uuid(&claims.subject, &state.pool)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(GetMeResponse::from(user)))
}

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

#[cfg(test)]
mod tests {
    use crate::handlers::testing::testing_server;
    use axum_test::TestServer;
    use serde_json::json;

    use super::*;

    /// Create a setup test server with default account to make testing easier
    async fn setup_test_server_with_default_account() -> TestServer {
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
        test_server
    }

    // My first ever axum sqlx test
    #[tokio::test]
    async fn post_register_no_error() {
        setup_test_server_with_default_account().await;
    }

    // Ensure duplicate emails are handled :3
    #[tokio::test]
    async fn post_register_duplicate_email_error() {
        let test_server = setup_test_server_with_default_account().await;

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
        let test_server = setup_test_server_with_default_account().await;

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

    #[tokio::test]
    async fn post_register_login_username_me() {
        let test_server = setup_test_server_with_default_account().await;
        let response = test_server
            .post("/api/auth/login")
            .json(&json!(PostLogin {
                username_or_email: "thairanaru".to_string(),
                password: "!testpassword123!".to_string(),
            }))
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let login_response: PostLoginResponse = response.json();

        let response = test_server
            .get("/api/auth/me")
            .authorization_bearer(login_response.access_token)
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);
        let me_body: GetMeResponse = response.json();

        // check if basic data matches
        // we probably want a comprehensive test later for all the fields later TODO
        assert_eq!(me_body.username, "thairanaru");
        assert_eq!(me_body.email, "thairanaru@proton.me");
    }

    #[tokio::test]
    async fn post_register_login_email() {
        let test_server = setup_test_server_with_default_account().await;
        let response = test_server
            .post("/api/auth/login")
            .json(&json!(PostLogin {
                username_or_email: "thairanaru@proton.me".to_string(),
                password: "!testpassword123!".to_string(),
            }))
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn post_register_login_bad_password() {
        let test_server = setup_test_server_with_default_account().await;
        let response = test_server
            .post("/api/auth/login")
            .json(&json!(PostLogin {
                username_or_email: "thairanaru@proton.me".to_string(),
                password: "bad_apple".to_string(),
            }))
            .await;

        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn post_me_bad_token() {
        let test_server = testing_server().await;
        let response = test_server
            .get("/api/auth/me")
            .authorization_bearer("fake_token")
            .await;
        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    }
}
