use futures::future::join_all;
use std::{cmp::Reverse, collections::HashMap};

use crate::{
    error::AppError,
    model::{SearchResult, Url},
    services::{urls::get_url, vector_embeding::Embeder},
};
use anyhow::anyhow;
use ordered_float::OrderedFloat;
use sqlx::{SqlitePool, query_as, query_scalar};

/// Search a query using FTS5 and vector similarity
pub async fn search_query(
    pool: &SqlitePool,
    query: &str,
    limit: u8,
    offset: u8,
    embeder: &mut Embeder,
) -> Result<Vec<Url>, AppError> {
    let fts5: Vec<String> = search_query_by_fts5(pool, query, limit + offset).await?;
    let vec_search: Vec<String> =
        search_query_by_vec_search(pool, query, limit + offset, embeder).await?;
    let rrf_results = rrf(fts5
        .iter()
        .enumerate()
        .chain(vec_search.iter().enumerate())
        .map(|(rank, string)| (rank, string.as_str())));
    let mut vec_result = Vec::from_iter(rrf_results);
    vec_result.sort_unstable_by_key(|(_, rank)| Reverse(OrderedFloat(*rank)));
    vec_result.drain(..offset as usize);
    join_all(vec_result.into_iter().map(|(url, _)| get_url(url, pool)))
        .await
        .into_iter()
        .map(|res| match res {
            Ok(Some(res)) => Ok(res),
            Ok(None) => Err(AppError::Anyhow(anyhow!(
                "Url found in virtuals tables not found in main tables"
            ))),
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<Url>, AppError>>()
}

const RRF_K: f32 = 60.0;
fn rrf<'a>(rankings: impl Iterator<Item = (usize, &'a str)>) -> HashMap<&'a str, f32> {
    // Since we know the combination of the two has to be at least 10.
    let mut res = HashMap::with_capacity(20);
    for (rank, url) in rankings {
        *res.entry(url).or_insert(0.0) += 1.0 / (RRF_K + rank as f32 + 1.0)
    }
    res
}

pub async fn search_query_by_vec_search(
    pool: &SqlitePool,
    query: &str,
    count: u8,
    embeder: &mut Embeder,
) -> Result<Vec<String>, AppError> {
    let embeding = embeder.embed(query)?;
    Ok(query_scalar(
        "
        SELECT url FROM vec_urls
        WHERE vec_urls.embeding MATCH ?
        ORDER BY distance
        LIMIT ?
        ",
    )
    .bind(bytemuck::cast_slice(&embeding))
    .bind(count as i64)
    .fetch_all(pool)
    .await
    .map_err(|e| anyhow!("database error: {}", e))?)
}

pub async fn search_query_by_fts5(
    pool: &SqlitePool,
    query: &str,
    count: u8,
) -> Result<Vec<String>, AppError> {
    Ok(query_scalar(
        "
        SELECT url FROM url_fts5_idx
        WHERE url_fts5_idx MATCH ?
        ORDER BY url_fts5_idx.rank
        LIMIT ?
        ",
    )
    .bind(into_match_query(query))
    .bind(count as i64)
    .fetch_all(pool)
    .await
    .map_err(|e| anyhow!("database error: {}", e))?)
}

pub fn into_match_query(query: &str) -> String {
    query.to_string()
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use crate::services::testing::setup_template_urls;

    use super::*;

    // in the future, we need tests that ensure that deleting and updating urls do not keep them in the virtual table TODO
    /// Checks if searching a empty pool gets back an empty result
    #[sqlx::test]
    async fn search_empty(pool: SqlitePool) {
        let mut embeder = Embeder::try_new().unwrap();
        assert_eq!(
            search_query(&pool, "Miku", 100, 0, &mut embeder)
                .await
                .unwrap(),
            Vec::<Url>::new()
        );
    }

    // TODO use testing pool for this rather than sqlx::test
    /// Checks if setting limit gets back nothing
    #[sqlx::test]
    async fn search_none(pool: SqlitePool) {
        let mut embeder = Embeder::try_new().unwrap();
        setup_template_urls(&pool, &mut embeder).await;
        assert_eq!(
            search_query(&pool, "Miku", 0, 0, &mut embeder)
                .await
                .unwrap(),
            vec![]
        );
    }

    /// Checks if search one matches expected result
    #[sqlx::test]
    async fn search_one(pool: SqlitePool) {
        let mut embeder = Embeder::try_new().unwrap();
        let res = setup_template_urls(&pool, &mut embeder).await;
        assert_eq!(
            search_query(&pool, "Miku", 1, 0, &mut embeder)
                .await
                .unwrap(),
            vec![res[0].clone()]
        );
    }

    /// Checks if offset works
    #[sqlx::test]
    async fn search_offset(pool: SqlitePool) {
        let mut embeder = Embeder::try_new().unwrap();
        let res = setup_template_urls(&pool, &mut embeder).await;
        assert_eq!(
            search_query(&pool, "Vocaloid OR Miku", 1, 1, &mut embeder)
                .await
                .unwrap(),
            vec![res[1].clone()]
        );
    }

    /// Checks if search multiple works
    #[sqlx::test]
    async fn search_multiples(pool: SqlitePool) {
        let mut embeder = Embeder::try_new().unwrap();
        let res = setup_template_urls(&pool, &mut embeder).await;
        assert_eq!(
            search_query(&pool, "Vocaloid OR Miku", 2, 0, &mut embeder)
                .await
                .unwrap(),
            vec![res[0].clone(), res[1].clone()]
        );
    }

    /// Check if vector search is running
    #[sqlx::test]
    async fn vector_search_example(pool: SqlitePool) {
        let mut embeder = Embeder::try_new().unwrap();
        let res = setup_template_urls(&pool, &mut embeder).await;
        let vec_search_output = search_query_by_vec_search(&pool, "Miku", 2, &mut embeder)
            .await
            .unwrap();
        assert_eq!(vec_search_output.len(), 2);
        assert_eq!(vec_search_output[0], res[0].clone().url);
        assert_eq!(vec_search_output[1], res[1].clone().url);
    }
}
