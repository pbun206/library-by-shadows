use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow)]
#[allow(dead_code)]
// the fields isn't truely dead anyway -> it's used for sql
pub struct User {
    pub uuid: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: i64,
    pub is_account_enabled: bool,
    pub web_theme: Theme,
    pub is_admin: bool,
}

#[derive(Deserialize, Serialize)]
pub enum Theme {
    System,
    Light,
    Dark,
}

impl From<String> for Theme {
    fn from(value: String) -> Self {
        match value.as_str() {
            "light" => Self::Light,
            "dark" => Self::Dark,
            _ => Self::System,
        }
    }
}

// Urls
#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct Url {
    pub url: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub first_indexed_at: i64,
    pub last_indexed_at: i64,
    pub last_published_at: Option<i64>,
    pub last_edited_at: Option<i64>,
}

impl Url {
    #[cfg(test)]
    pub fn with_default_metadata(
        url: String,
        title: String,
        description: String,
        content: String,
    ) -> Url {
        Url {
            url,
            title,
            description,
            content,
            first_indexed_at: 0,
            last_indexed_at: 0,
            last_published_at: None,
            last_edited_at: None,
        }
    }
}

/// Search result
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SearchResult {
    pub url: String,
    pub title: String,
    pub description: String,
    pub last_published_at: Option<i64>,
    pub last_edited_at: Option<i64>,
}

impl From<Url> for SearchResult {
    fn from(url: Url) -> Self {
        Self {
            url: url.url,
            title: url.title,
            description: url.description,
            last_published_at: url.last_published_at,
            last_edited_at: url.last_edited_at,
        }
    }
}
