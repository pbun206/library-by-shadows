use std::sync::Arc;
use tokio::sync::RwLock;

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::{AppState, error::AppError, services::accounts::create_user};

pub async fn get_me() {}

#[derive(Deserialize, Serialize)]
pub struct PostRegister {
    username: String,
    email: String,
    password: String,
}

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

pub async fn post_login() {}

pub async fn post_logout() {}

#[cfg(test)]
mod tests {
    use crate::route::create_router;
    use axum::{body::Body, extract::Request};
    use serde_json::json;
    use sqlx::SqlitePool;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use tower::ServiceExt;

    use super::*;

    // My first ever axum sqlx test
    #[sqlx::test]
    fn post_register_no_error(pool: SqlitePool) {
        let app = Arc::new(RwLock::new(AppState::with_test_config(pool)));
        let router = create_router(app);
        let response = router
            .oneshot(
                Request::post("/api/auth/register")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!(PostRegister {
                            username: "thairanaru".to_string(),
                            email: "thairanaru@proton.me".to_string(),
                            password: "!testpassword123!".to_string(),
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    // Ensure duplicate emails are handled :3
    #[sqlx::test]
    fn post_register_duplicate_email_error(pool: SqlitePool) {
        let app = Arc::new(RwLock::new(AppState::with_test_config(pool)));
        let router = create_router(app);
        let response = router
            .clone()
            .oneshot(
                Request::post("/api/auth/register")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!(PostRegister {
                            username: "thairanaru".to_string(),
                            email: "thairanaru@proton.me".to_string(),
                            password: "!testpassword123!".to_string(),
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let response = router
            .clone()
            .oneshot(
                Request::post("/api/auth/register")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!(PostRegister {
                            username: "naruthaira".to_string(),
                            email: "thairanaru@proton.me".to_string(),
                            password: "!testpassword123!".to_string(),
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    // Ensure duplicate emails are handled :3
    #[sqlx::test]
    fn post_register_duplicate_username_error(pool: SqlitePool) {
        let app = Arc::new(RwLock::new(AppState::with_test_config(pool)));
        let router = create_router(app);
        let response = router
            .clone()
            .oneshot(
                Request::post("/api/auth/register")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!(PostRegister {
                            username: "thairanaru".to_string(),
                            email: "thairanaru@proton.me".to_string(),
                            password: "!testpassword123!".to_string(),
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let response = router
            .clone()
            .oneshot(
                Request::post("/api/auth/register")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!(PostRegister {
                            username: "thairanaru".to_string(),
                            email: "naruthaira@proton.me".to_string(),
                            password: "!testpassword123!".to_string(),
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }
}
