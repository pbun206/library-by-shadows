use std::sync::Arc;

use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    AppState,
    error::AppError,
    model::{Theme, User},
};

#[derive(Deserialize, Serialize)]
pub struct GetMeResponse {
    pub uuid: String,
    pub username: String,
    pub email: String,
    pub created_at: i64,
    pub is_account_enabled: bool,
    pub web_theme: Theme,
    pub is_admin: bool,
}

impl From<User> for GetMeResponse {
    fn from(user: User) -> Self {
        Self {
            uuid: user.uuid,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            is_account_enabled: user.is_account_enabled,
            web_theme: user.web_theme,
            is_admin: user.is_admin,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct PostRegister {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct PostLogin {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostLoginResponse {
    pub access_token: String,
    pub token_type: String,
}

impl PostLoginResponse {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub subject: String,
    pub exp: usize,
}

//https://github.com/tokio-rs/axum/blob/main/examples/jwt/src/main.rs
impl FromRequestParts<Arc<RwLock<AppState>>> for Claims {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        app_state: &Arc<RwLock<AppState>>,
    ) -> Result<Self, Self::Rejection> {
        let state = app_state.read().await;
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Unauthorized)?;
        // Decode the user data
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized)?;

        Ok(token_data.claims)
    }
}
