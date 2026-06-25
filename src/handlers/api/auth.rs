use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use tower_sessions::Session;

use crate::{AppState, services::accounts::create_user};

pub async fn get_me() {}

#[derive(Deserialize)]
struct PostRegister {
    username: String,
    email: String,
    password: String,
}
pub async fn post_register(
    State(app_state): State<AppState>,
    session: Session,
    Json(payload): Json<PostRegister>,
) -> StatusCode {
    create_user(
        payload.email,
        payload.password,
        payload.username,
        &app_state.pool,
    );
    todo!();
}

pub async fn post_login() {}

pub async fn post_logout() {}
