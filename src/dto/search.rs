use serde::{Deserialize, Serialize};

use crate::model::SearchResult;

/// Get Url query
#[derive(Deserialize)]
pub struct GetSearchQuery {
    query: String,
    limit: u8,
    offset: u8,
}

/// Body for search results
#[derive(Deserialize, Serialize)]
pub struct GetSearchResponse {
    results: Vec<SearchResult>,
}
