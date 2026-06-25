use anyhow::{Result, anyhow, bail};
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use sqlx::{SqlitePool, query_as, query_scalar};
use uuid::Uuid;

use crate::model::User;

pub async fn create_user(
    email: String,
    password: String,
    username: String,
    pool: &SqlitePool,
) -> Result<User> {
    // Check if the email is already in use
    let email_taken: bool = query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = ?)")
        .bind(email.to_ascii_lowercase())
        .fetch_one(pool)
        .await
        .unwrap_or(false);
    if email_taken {
        bail!("email already in use.");
    }

    // Also username
    let username_taken: bool =
        query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE username = ?)")
            .bind(&username)
            .fetch_one(pool)
            .await
            .unwrap_or(false);
    if username_taken {
        bail!("username already taken.");
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!("failed to hash password: {}", e))
        .map(|hash| hash.to_string())?;

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

// pub async fn check_email_password(
//     email: String,
//     password: String,
//     pool: &SqlitePool,
// ) -> Result<User> {
//     let email = email.to_ascii_lowercase();
//     let user = query_as!(User, "SELECT * FROM users WHERE email = $1", email)
//         .fetch_optional(pool)
//         .await
//         .map_err(|e| anyhow!("database error: {}.", e))?
//         .ok_or_else(|| anyhow!("invalid email or password."))?;

//     let is_valid = match PasswordHash::new(&user.password) {
//         Ok(parsed_hash) => Argon2::default()
//             .verify_password(password.as_bytes(), &parsed_hash)
//             .map_or(false, |_| true),
//         Err(_err) => false,
//     };
//     w

//     if !is_valid {
//         bail!("invalid email or password.");
//     }

//     Ok(user)
// }
