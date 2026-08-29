use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Query, State};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    AppState, dto::search::GetSearchQuery, error::AppError, model::SearchResult,
    services::search::search_query,
};

#[derive(Template, WebTemplate)]
#[template(path = "search_results.html")]
pub struct SearchResultsTemplate {
    search_field_value: String,
    results: Vec<SearchResult>,
}

pub async fn get_search_page(
    State(app_state): State<Arc<RwLock<AppState>>>,
    query: Query<GetSearchQuery>,
) -> Result<SearchResultsTemplate, AppError> {
    let state = &mut *app_state.write().await;
    let query = query.0;
    Ok(SearchResultsTemplate {
        results: search_query(
            &state.pool,
            &query.query,
            query.limit.unwrap_or(20),
            query.offset.unwrap_or(0),
            &mut state.embeder,
        )
        .await?
        .into_iter()
        .map(|url| url.into())
        .collect(),
        search_field_value: query.query,
    })
}

#[derive(Template, WebTemplate)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    search_field_value: String,
}

pub async fn home(
    State(app_state): State<Arc<RwLock<AppState>>>,
) -> Result<HomeTemplate, AppError> {
    Ok(HomeTemplate {
        search_field_value: String::new(),
    })
}
