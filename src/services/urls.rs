use anyhow::{Result, anyhow, bail};
use sqlx::{SqlitePool, query_as, query_scalar};

use crate::{
    error::AppError,
    model::{Url, User},
    services::vector_embeding::Embeder,
};

/// Adds a url from database :3
pub async fn add_url(
    url: String,
    title: String,
    description: String,
    content: String,
    pool: &SqlitePool,
    embeder: &mut Embeder,
) -> Result<Url, AppError> {
    let embeding = embeder.embed(&description, &content)?;
    let url = query_as!(
        Url,
        "INSERT INTO urls (url,title, description, content) VALUES (?, ?, ?, ?) RETURNING *",
        url,
        title,
        description,
        content,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| anyhow!("database error: {}", e))?;
    query_as!(
        Url,
        "INSERT INTO vec_urls (url, embeding) VALUES (?, ?, ?, ?) RETURNING *",
        url,
        title,
        description,
        content,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| anyhow!("database error: {}", e))?;

    Ok(url)
}

/// Update a url from database :3
pub async fn update_url(
    url: String,
    title: String,
    description: String,
    content: String,
    pool: &SqlitePool,
) -> Result<(), AppError> {
    query_as!(
        URL,
        "UPDATE urls SET title = ?, description = ?, content = ? WHERE url = ?",
        title,
        description,
        content,
        url,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| anyhow!("database error: {}", e))?;

    Ok(())
}

/// Get a url from database :3. None -> not there
pub async fn get_url(url: String, pool: &SqlitePool) -> Result<Option<Url>, AppError> {
    let url = query_as!(Url, "Select * FROM urls where url = ?", url,)
        .fetch_optional(pool)
        .await
        .map_err(|e| anyhow!("database error: {}", e))?;
    Ok(url)
}

/// Delete a url from database :<
pub async fn delete_url(url: String, pool: &SqlitePool) -> Result<(), AppError> {
    query_as!(Url, "DELETE FROM urls where url = ? RETURNING *", url,)
        .fetch_optional(pool)
        .await
        .map_err(|e| anyhow!("database error: {}", e))?
        .ok_or(AppError::NotFound)?;
    Ok(())
}
