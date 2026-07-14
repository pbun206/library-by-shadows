use serde::{Deserialize, Serialize};

use crate::model::SearchResult;

/// Get Url query
#[derive(Deserialize)]
pub struct GetSearchQuery {
    pub query: String,
    pub limit: Option<u8>,
    pub offset: Option<u8>,
}

/// Body for search results
#[derive(Deserialize, Serialize)]
pub struct GetSearchResponse {
    results: Vec<SearchResult>,
}
