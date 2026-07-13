use serde::{Deserialize, Serialize};


/// Get Url Response body
#[derive(Deserialize, Serialize)]
pub struct GetUrlResponse {
    pub url: String,
    pub title: String,
    pub description: String,
    pub first_indexed_at: i64,
    pub last_indexed_at: i64,
    pub last_published_at: Option<i64>,
    pub last_edited_at: Option<i64>,
}

/// Get Url query
#[derive(Deserialize)]
pub struct UrlQuery {
    pub url: String,
}


/// Post Url Request Bos
#[derive(Deserialize, Serialize)]
pub struct PostUrlRequest {
    pub url: String,
    pub title: String,
    pub description: String,
    pub content: String,
}
