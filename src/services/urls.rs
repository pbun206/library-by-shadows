use anyhow::{Result, anyhow};
use sqlx::{SqlitePool, query, query_as};

use crate::{error::AppError, model::Url, services::vector_embeding::Embeder};

/// Adds a url from database :3
pub async fn add_url(
    url_string: String,
    title: String,
    description: String,
    content: String,
    pool: &SqlitePool,
    embeder: &mut Embeder,
) -> Result<Url, AppError> {
    let embeding = embeder.embed(&(String::from(&description) + " " + &content))?;
    let url = query_as!(
        Url,
        "INSERT INTO urls (url,title, description, content) VALUES (?, ?, ?, ?) RETURNING *",
        url_string,
        title,
        description,
        content,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| anyhow!("database error: {}", e))?;

    // vector emebeding
    query("INSERT INTO vec_urls (url, embeding) VALUES (?, ?)")
        .bind(url_string)
        .bind(bytemuck::cast_slice(&embeding))
        .execute(pool)
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
    embeder: &mut Embeder,
) -> Result<(), AppError> {
    let embeding = embeder.embed(&(String::from(&description) + " " + &content))?;
    query!(
        "UPDATE urls SET title = ?, description = ?, content = ? WHERE url = ?",
        title,
        description,
        content,
        url
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!("database error: {}", e))?;

    // vector emebeding
    query("UPDATE vec_urls SET embeding = ? WHERE url = ?")
        .bind(bytemuck::cast_slice(&embeding))
        .bind(url)
        .execute(pool)
        .await
        .map_err(|e| anyhow!("database error: {}", e))?;

    Ok(())
}

/// Get a url from database :3. None -> not there
pub async fn get_url(url: &str, pool: &SqlitePool) -> Result<Option<Url>, AppError> {
    let url = query_as!(Url, "Select * FROM urls where url = ?", url,)
        .fetch_optional(pool)
        .await
        .map_err(|e| anyhow!("database error: {}", e))?;
    Ok(url)
}

/// Delete a url from database :<
// TODO use a trigger to make vec url deltion automatic
pub async fn delete_url(url: String, pool: &SqlitePool) -> Result<(), AppError> {
    query!("DELETE FROM urls where url = ? RETURNING *", url,)
        .fetch_optional(pool)
        .await
        .map_err(|e| anyhow!("database error: {}", e))?
        .ok_or(AppError::NotFound)?;
    query("DELETE FROM vec_urls where url = ?")
        .bind(url)
        .execute(pool)
        .await
        .map_err(|e| anyhow!("database error: {}", e))?;
    Ok(())
}
