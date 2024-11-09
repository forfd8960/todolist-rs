use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::errors::AppError;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,

    #[sqlx(default)]
    #[serde(skip)]
    pub password_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

pub async fn create_user(input: &CreateUser, pool: &PgPool) -> Result<User, AppError> {
    let user_result = get_user_by_email(&input.email, pool).await?;
    if user_result.is_some() {
        return Err(AppError::EmailAlreadyExists(format!(
            "{}",
            input.email.clone()
        )));
    }

    let user = sqlx::query_as("INSERT INTO users(username, email, password_hash) VALUES($1, $2, $3) RETURNING id,username,email,created_at,updated_at")
        .bind(&input.username)
        .bind(&input.email)
        .bind(&input.password_hash)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn get_user_by_email(email: &str, pool: &PgPool) -> Result<Option<User>, AppError> {
    let user =
        sqlx::query_as("SELECT id,username,email,created_at,updated_at FROM users WHERE email=$1")
            .bind(email)
            .fetch_optional(pool)
            .await?;
    Ok(user)
}

pub async fn get_user_by_id(id: i64, pool: &PgPool) -> Result<Option<User>, AppError> {
    let user =
        sqlx::query_as("SELECT id,username,email,created_at,updated_at FROM users WHERE id=$1")
            .bind(id)
            .fetch_optional(pool)
            .await?;
    Ok(user)
}
