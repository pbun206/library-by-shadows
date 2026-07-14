use crate::{
    error::AppError,
    model::{SearchResult, Url},
};
use anyhow::anyhow;
use sqlx::{SqlitePool, query_as};

/// Search a query using FTS5
// TODO density search
// TODO rff
pub async fn search_query(
    pool: &SqlitePool,
    query: &str,
    limit: u8,
    offset: u8,
) -> Result<Vec<Url>, AppError> {
    let res: Vec<Url> = query_as(
        "
        SELECT urls.url, urls.title, urls.description, urls.content, urls.first_indexed_at, urls.last_indexed_at, urls.last_published_at, urls.last_edited_at
        FROM urls
        INNER JOIN url_fts5_idx ON url_fts5_idx.url = urls.url
        WHERE url_fts5_idx MATCH ?
        ORDER BY url_fts5_idx.rank
        LIMIT ?
        OFFSET ?;
        ",
    )
        .bind(query)
        .bind(limit as i64)
        .bind(offset as i64)
    .fetch_all(pool)
    .await
    .map_err(|e| anyhow!("database error: {}", e))?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use crate::services::testing::{setup_template_urls, template_urls};

    use super::*;

    /// Checks if searching a empty pool gets back an empty result
    #[sqlx::test]
    async fn search_empty(pool: SqlitePool) {
        assert_eq!(search_query(&pool, "Miku", 100, 0).await.unwrap(), vec![]);
    }

    /// Checks if setting limit gets back nothing
    #[sqlx::test]
    async fn search_none(pool: SqlitePool) {
        setup_template_urls(&pool).await;
        assert_eq!(search_query(&pool, "Miku", 0, 0).await.unwrap(), vec![]);
    }

    /// Checks if search one matches expected result
    #[sqlx::test]
    async fn search_one(pool: SqlitePool) {
        let res = setup_template_urls(&pool).await;
        assert_eq!(
            search_query(&pool, "Miku", 1, 0).await.unwrap(),
            vec![res[0].clone()]
        );
    }

    /// Checks if offset works
    #[sqlx::test]
    async fn search_offset(pool: SqlitePool) {
        let res = setup_template_urls(&pool).await;
        assert_eq!(
            search_query(&pool, "Vocaloid OR Miku", 1, 1).await.unwrap(),
            vec![res[1].clone()]
        );
    }

    /// Checks if search multiple works
    #[sqlx::test]
    async fn search_multiples(pool: SqlitePool) {
        let res = setup_template_urls(&pool).await;
        assert_eq!(
            search_query(&pool, "Vocaloid OR Miku", 2, 0).await.unwrap(),
            vec![res[0].clone(), res[1].clone()]
        );
    }
}
