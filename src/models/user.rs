use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::errors::AppError;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, PartialEq)]
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
    let user = sqlx::query_as(
        "SELECT id,username,email,password_hash,created_at,updated_at FROM users WHERE email=$1",
    )
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_create_user_works() -> Result<()> {
        let pool =
            PgPool::connect("postgres://db_manager:super_admin8801@localhost:5432/todolist_test")
                .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        let user = create_user(
            &CreateUser {
                username: "Bob".to_string(),
                email: "bob@acme.org".to_string(),
                password_hash: "test".to_string(),
            },
            &pool,
        )
        .await?;

        assert_eq!(user.username, "Bob".to_string());
        assert_eq!(user.email, "bob@acme.org".to_string());
        assert_eq!(user.password_hash, None);

        sqlx::query("TRUNCATE TABLE users,todos;")
            .execute(&pool)
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_user_by_email_work() -> Result<()> {
        let pool =
            PgPool::connect("postgres://db_manager:super_admin8801@localhost:5432/todolist_test")
                .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        let user = create_user(
            &CreateUser {
                username: "Bob".to_string(),
                email: "bob@acme.org".to_string(),
                password_hash: "test".to_string(),
            },
            &pool,
        )
        .await?;

        let user2 = get_user_by_email("bob@acme.org", &pool).await?;
        assert!(user2.is_some());
        assert_eq!(user, user2.unwrap());

        sqlx::query("TRUNCATE TABLE users,todos;")
            .execute(&pool)
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_user_by_id_work() -> Result<()> {
        let pool =
            PgPool::connect("postgres://db_manager:super_admin8801@localhost:5432/todolist_test")
                .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        let user = create_user(
            &CreateUser {
                username: "Bob".to_string(),
                email: "bob@acme.org".to_string(),
                password_hash: "test".to_string(),
            },
            &pool,
        )
        .await?;

        let user2 = get_user_by_id(user.id, &pool).await?;
        assert!(user2.is_some());
        assert_eq!(user, user2.unwrap());

        sqlx::query("TRUNCATE TABLE users,todos;")
            .execute(&pool)
            .await?;

        Ok(())
    }
}
