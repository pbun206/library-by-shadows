use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{AppState, error::AppError, services::urls::add_url};

pub async fn get_url(
    State(app_state): State<Arc<RwLock<AppState>>>,
) -> Result<StatusCode, AppError> {
    let state = app_state.read().await;
    get_url(
        payload.url,
        payload.title,
        payload.description,
        payload.content,
        &state.pool,
    )
    .await?;
    Ok(StatusCode::CREATED)
}

#[derive(Deserialize, Serialize)]
pub struct PostUrl {
    url: String,
    title: String,
    description: String,
    content: String,
}

pub async fn post_url(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Json(payload): Json<PostUrl>,
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

pub async fn delete_url() {}
