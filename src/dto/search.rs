use serde::{Deserialize, Serialize};

/// Get Url query
#[derive(Deserialize)]
pub struct GetSearchQuery {
    query: String,
    start: u8,
    end: u8,
}

/// Body for search results
#[derive(Deserialize, Serialize)]
pub struct GetSearchResponse {
    results: Vec<SearchResult>,
}

/// Search response
#[derive(Deserialize, Serialize)]
pub struct SearchResult {
    pub url: String,
    pub title: String,
    pub description: String,
    pub last_published_at: Option<i64>,
    pub last_edited_at: Option<i64>,
}
