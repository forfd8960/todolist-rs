use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::errors::AppError;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub description: String,
    pub status: i8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub user_id: i64,
    pub title: String,
    pub description: String,
    pub status: i8,
}

pub async fn create_todo(input: &CreateTodo, pool: &PgPool) -> Result<Todo, AppError> {
    let todo = sqlx::query_as(
        r#"INSERT INTO todos(user_id, title, description, status)
                VALUES($1, $2, #3, $4) 
                RETURNING id,user_id,title,description,status,created_at,updated_at
                "#,
    )
    .bind(&input.user_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(input.status)
    .fetch_one(pool)
    .await?;

    Ok(todo)
}

pub async fn get_todo_by_id(id: i64, pool: &PgPool) -> Result<Option<Todo>, AppError> {
    let todo = sqlx::query_as(
        r#"
        SELECT id,user_id,title,description,status,created_at,updated_at
        FROM todos
        WHERE id=$1;
    "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(todo)
}

pub async fn get_todos_by_user_id(user_id: i64, pool: &PgPool) -> Result<Vec<Todo>, AppError> {
    let todos = sqlx::query_as(
        r#"
        SELECT id,user_id,title,description,status,created_at,updated_at
        FROM todos
        WHERE user_id=$1
        ORDER BY created_at DESC;
    "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(todos)
}

pub async fn update_todo_status(
    id: i64,
    status: i8,
    pool: &PgPool,
) -> Result<Option<Todo>, AppError> {
    let todo = get_todo_by_id(id, pool).await?;
    if todo.is_none() {
        return Err(AppError::TodoNotExists(format!("{} is not exists", id)));
    }

    let todo: Todo = sqlx::query_as(
        r#"
        UPDATE todos SET status=$1 WHERE id=$2
        RETURNING id,user_id,title,description,status,created_at,updated_at;
    "#,
    )
    .bind(status)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(Some(todo))
}
