use anyhow::{Result, anyhow};
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use sqlx::{SqlitePool, query_as, query_scalar};
use uuid::Uuid;

use crate::{error::AppError, model::User};

pub async fn create_user(
    email: &str,
    password: &str,
    username: &str,
    pool: &SqlitePool,
    pepper: &str,
) -> Result<User, AppError> {
    // Check if the email is already in use
    let email_taken: bool = query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = ?)")
        .bind(email.to_ascii_lowercase())
        .fetch_one(pool)
        .await
        .unwrap_or(false);
    if email_taken {
        return Err(AppError::Conflict);
    }

    if !is_username_valid(username) {
        return Err(AppError::Conflict);
    }

    // Also username
    let username_taken: bool =
        query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE username = ?)")
            .bind(username)
            .fetch_one(pool)
            .await
            .unwrap_or(false);
    if username_taken {
        return Err(AppError::Conflict);
    }

    let password_hash = Argon2::new_with_secret(
        pepper.as_bytes(),
        Algorithm::default(),
        Version::default(),
        Params::default(),
    )
    .map_err(|e| anyhow!("Canot make argon: {}", e))?
    .hash_password(password.as_bytes())
    .map_err(|e| anyhow!("failed to hash password: {}", e))?
    .to_string();

    let uuid = Uuid::new_v4().to_string();

    let user = query_as!(
        User,
        "INSERT INTO users (uuid,username, email, password_hash) VALUES (?, ?, ?, ?) RETURNING *",
        uuid,
        username,
        email.to_ascii_lowercase(),
        password_hash,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| anyhow!("database error: {}", e))?;

    Ok(user)
}

// TODO check for any weird characters like @
fn is_username_valid(user: &str) -> bool {
    !user.contains("@")
}

/// Get a user from database from username
pub async fn get_user_from_username(
    username: &str,
    pool: &SqlitePool,
) -> Result<Option<User>, AppError> {
    let user = query_as!(User, "Select * FROM users where username = ?", username,)
        .fetch_optional(pool)
        .await
        .map_err(|e| anyhow!("database error: {}", e))?;
    Ok(user)
}

/// Get a user from database from email
pub async fn get_user_from_email(email: &str, pool: &SqlitePool) -> Result<Option<User>, AppError> {
    let user = query_as!(User, "Select * FROM users where email = ?", email,)
        .fetch_optional(pool)
        .await
        .map_err(|e| anyhow!("database error: {}", e))?;
    Ok(user)
}

/// Get a user from database from uuid
pub async fn get_user_from_uuid(uuid: &str, pool: &SqlitePool) -> Result<Option<User>, AppError> {
    let user = query_as!(User, "Select * FROM users where uuid = ?", uuid,)
        .fetch_optional(pool)
        .await
        .map_err(|e| anyhow!("database error: {}", e))?;
    Ok(user)
}

pub fn does_hash_match_password(hash: &str, password: &str, pepper: &str) -> Result<(), AppError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| anyhow!("argon error: {}", e))?;
    Argon2::new_with_secret(
        pepper.as_bytes(),
        Algorithm::default(),
        Version::default(),
        Params::default(),
    )
    .map_err(|e| anyhow!("argon error: {}", e))?
    .verify_password(password.as_bytes(), &parsed_hash)
    .map_err(|e| match e {
        argon2::password_hash::Error::PasswordInvalid => AppError::Unauthorized,
        _ => anyhow!("argon error: {}", e).into(),
    })
}

pub async fn authenticate_user(
    username_or_email: &str,
    password: &str,
    pool: &SqlitePool,
    pepper: &str,
) -> Result<User, AppError> {
    // Find username or email was inputted.
    let is_username = is_username_valid(username_or_email);
    let user = if is_username {
        get_user_from_username(username_or_email, pool).await
    } else {
        get_user_from_email(username_or_email, pool).await
    }?
    .ok_or(AppError::Unauthorized)?;

    does_hash_match_password(&user.password_hash, password, pepper)?;
    Ok(user)
}
