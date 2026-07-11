use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{AppState, error::AppError, services::urls::add_url};

#[derive(Deserialize, Serialize)]
pub struct GetUrlResponse {
    url: String,
    pub title: String,
    pub description: String,
    pub first_indexed_at: i64,
    pub last_indexed_at: i64,
    pub last_published_at: Option<i64>,
    pub last_edited_at: Option<i64>,
}

#[derive(Deserialize)]
pub struct UrlQuery {
    url: String,
}
pub async fn get_url(
    State(app_state): State<Arc<RwLock<AppState>>>,
    query: Query<UrlQuery>,
) -> Result<Json<GetUrlResponse>, AppError> {
    let state = app_state.read().await;
    let internal_url = crate::services::urls::get_url(query.0.url, &state.pool)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(GetUrlResponse {
        url: internal_url.url,
        title: internal_url.title,
        description: internal_url.description,
        first_indexed_at: internal_url.first_indexed_at,
        last_indexed_at: internal_url.last_indexed_at,
        last_published_at: internal_url.last_published_at,
        last_edited_at: internal_url.last_edited_at,
    }))
}

#[derive(Deserialize, Serialize)]
pub struct PostUrlRequest {
    url: String,
    title: String,
    description: String,
    content: String,
}

pub async fn post_url(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Json(payload): Json<PostUrlRequest>,
) -> Result<StatusCode, AppError> {
    let state = app_state.read().await;
    add_url(
        payload.url,
        payload.title,
        payload.description,
        payload.content,
        &state.pool,
    )
    .await?;
    Ok(StatusCode::CREATED)
}

pub async fn delete_url(
    State(app_state): State<Arc<RwLock<AppState>>>,
    query: Query<UrlQuery>,
) -> Result<StatusCode, AppError> {
    let state = app_state.read().await;
    crate::services::urls::delete_url(query.0.url, &state.pool).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use crate::{AppState, handlers::testing::testing_server, route::create_router};
    use axum::{body::Body, extract::Request};
    use serde_json::json;
    use sqlx::SqlitePool;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use tower::ServiceExt;

    use super::*;

    // Post url
    #[sqlx::test]
    fn post_url(pool: SqlitePool) {
        let test_server = testing_server(pool);
        let response = test_server
            .post("/api/index/url")
            .json(&json!(PostUrlRequest {
                url: "ec.crypton.co.jp/pages/prod/virtualsinger/cv01_us".to_string(),                title: "About HATSUNE MIKU | CRYPTON FUTURE MEDIA".to_string(),
                description: "She is a singing voice synthesizer featured in over 100,000 songs released worldwide. Hatsune Miku means The first Sound from the Future.".to_string(),
                content: "<Insert content about Miku Here>".to_string(),
            }))
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
    }

    // Post and get url
    #[sqlx::test]
    fn post_url_and_get(pool: SqlitePool) {
        let test_server = testing_server(pool);
        let response = test_server
            .post("/api/index/url")
            .json(&json!(PostUrlRequest {
                url: "ec.crypton.co.jp/pages/prod/virtualsinger/cv01_us".to_string(),                title: "About HATSUNE MIKU | CRYPTON FUTURE MEDIA".to_string(),
                description: "She is a singing voice synthesizer featured in over 100,000 songs released worldwide. Hatsune Miku means The first Sound from the Future.".to_string(),
                content: "<Insert content about Miku Here>".to_string(),
            }))
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);

        let response = test_server
            .get("/api/index/url?url=ec.crypton.co.jp/pages/prod/virtualsinger/cv01_us")
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: GetUrlResponse = response.json();
        assert_eq!(
            body.url,
            "ec.crypton.co.jp/pages/prod/virtualsinger/cv01_us"
        );
        assert_eq!(body.title, "About HATSUNE MIKU | CRYPTON FUTURE MEDIA");
        assert_eq!(
            body.description,
            "She is a singing voice synthesizer featured in over 100,000 songs released worldwide. Hatsune Miku means The first Sound from the Future."
        );
    }

    // Post, delete, and get a url
    #[sqlx::test]
    fn post_url_delete_and_get(pool: SqlitePool) {
        let test_server = testing_server(pool);
        let response = test_server
            .post("/api/index/url")
            .json(&json!(PostUrlRequest {
                url: "ec.crypton.co.jp/pages/prod/virtualsinger/cv01_us".to_string(),                title: "About HATSUNE MIKU | CRYPTON FUTURE MEDIA".to_string(),
                description: "She is a singing voice synthesizer featured in over 100,000 songs released worldwide. Hatsune Miku means The first Sound from the Future.".to_string(),
                content: "<Insert content about Miku Here>".to_string(),
            }))
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);

        let response = test_server
            .delete("/api/index/url?url=ec.crypton.co.jp/pages/prod/virtualsinger/cv01_us")
            .await;
        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);


        let response = test_server
            .get("/api/index/url?url=ec.crypton.co.jp/pages/prod/virtualsinger/cv01_us")
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }
}
