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
    pub status: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub enum TodoStatus {
    Pending,
    Ready,
    InProgress,
    Done,
}

impl TodoStatus {
    pub fn get_status_id(&self) -> i16 {
        match self {
            TodoStatus::Pending => 0,
            TodoStatus::Ready => 1,
            TodoStatus::InProgress => 2,
            TodoStatus::Done => 3,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub user_id: i64,
    pub title: String,
    pub description: String,
    pub status: i16,
}

#[derive(Debug)]
pub struct GetTodosArgs {
    pub user_id: i64,
    pub offset: i64,
    pub limit: i64,
    pub status: Option<i16>,
}

pub async fn create_todo(input: &CreateTodo, pool: &PgPool) -> Result<Todo, AppError> {
    let todo = sqlx::query_as(
        r#"INSERT INTO todos (user_id,title,description,status)
                VALUES ($1,$2,$3,$4)
                RETURNING id,user_id,title,description,status,created_at,updated_at;
                "#,
    )
    .bind(input.user_id)
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

pub async fn get_todos_by_user_id(
    args: GetTodosArgs,
    pool: &PgPool,
) -> Result<Vec<Todo>, AppError> {
    let mut offset = args.offset;
    if args.offset < 0 {
        offset = 0;
    }

    let mut limit = args.limit;
    if args.limit <= 0 || args.limit > 100 {
        limit = 100
    }

    let mut status = TodoStatus::Pending.get_status_id();
    if args.status.is_some() {
        status = args.status.unwrap();
    }

    let todos = sqlx::query_as(
        r#"
        SELECT id,user_id,title,description,status,created_at,updated_at
        FROM todos
        WHERE user_id=$1 AND status=$2
        ORDER BY id DESC
        LIMIT $3
        OFFSET $4;
    "#,
    )
    .bind(args.user_id)
    .bind(status)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(todos)
}

pub async fn update_todo_status(id: i64, status: i16, pool: &PgPool) -> Result<Todo, AppError> {
    let todo = get_todo_by_id(id, pool).await?;
    if todo.is_none() {
        return Err(AppError::TodoNotExists(format!("{} is not exists", id)));
    }

    let todo = sqlx::query_as(
        r#"
        UPDATE todos SET status=$1 WHERE id=$2
        RETURNING id,user_id,title,description,status,created_at,updated_at;
    "#,
    )
    .bind(status)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(todo)
}

#[cfg(test)]
mod tests {
    use crate::models::user::{create_user, CreateUser};

    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_create_todo() -> Result<()> {
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

        let todo = create_todo(
            &CreateTodo {
                user_id: user.id,
                title: "axum-tutorial".to_string(),
                description: "1: write doc, 2: make video".to_string(),
                status: TodoStatus::Pending.get_status_id(),
            },
            &pool,
        )
        .await?;
        assert_eq!(todo.title, "axum-tutorial".to_string());
        assert_eq!(todo.description, "1: write doc, 2: make video".to_string());
        assert_eq!(todo.status, TodoStatus::Pending.get_status_id());
        assert_eq!(todo.user_id, user.id);

        sqlx::query("TRUNCATE TABLE users,todos;")
            .execute(&pool)
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_todo_by_id() -> Result<()> {
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

        let todo = create_todo(
            &CreateTodo {
                user_id: user.id,
                title: "axum-tutorial".to_string(),
                description: "1: write doc, 2: make video".to_string(),
                status: TodoStatus::Pending.get_status_id(),
            },
            &pool,
        )
        .await?;

        let todo_res = get_todo_by_id(todo.id, &pool).await?;
        assert!(todo_res.is_some());

        let todo = todo_res.unwrap();
        assert_eq!(todo.title, "axum-tutorial".to_string());
        assert_eq!(todo.description, "1: write doc, 2: make video".to_string());
        assert_eq!(todo.status, TodoStatus::Pending.get_status_id());
        assert_eq!(todo.user_id, user.id);
        assert_eq!(todo.id, todo.id);

        sqlx::query("TRUNCATE TABLE users,todos;")
            .execute(&pool)
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_update_todo_status() -> Result<()> {
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

        let todo = create_todo(
            &CreateTodo {
                user_id: user.id,
                title: "axum-tutorial".to_string(),
                description: "1: write doc, 2: make video".to_string(),
                status: TodoStatus::Pending.get_status_id(),
            },
            &pool,
        )
        .await?;

        let new_status = TodoStatus::Ready.get_status_id();

        let todo = update_todo_status(todo.id, new_status, &pool).await?;

        assert_eq!(todo.title, "axum-tutorial".to_string());
        assert_eq!(todo.description, "1: write doc, 2: make video".to_string());
        assert_eq!(todo.status, new_status);
        assert_eq!(todo.user_id, user.id);
        assert_eq!(todo.id, todo.id);

        sqlx::query("TRUNCATE TABLE users,todos;")
            .execute(&pool)
            .await?;

        Ok(())
    }
}
