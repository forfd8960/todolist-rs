use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use tracing::info;

use crate::{
    errors::AppError,
    models::todolist::{self, CreateTodo, GetTodosArgs},
    AppState,
};

use super::request::{CreateTodoReq, CreateTodoResp, ListTodosReq, ListTodosResp, UpdateTodosReq};

#[axum::debug_handler]
pub async fn create_todo_handler(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    Json(req): Json<CreateTodoReq>,
) -> Result<impl IntoResponse, AppError> {
    info!("{:?}", req);

    let token = bearer.token();
    let user = state.auth.verify_user(token)?;
    let create_todo = CreateTodo {
        user_id: user.id,
        title: req.title,
        description: req.description,
        status: req.status.get_status_id(),
    };

    let todo = todolist::create_todo(&create_todo, &state.pool).await?;

    info!("created todo: {:?}", todo);

    Ok((StatusCode::OK, Json(CreateTodoResp { todo })).into_response())
}

#[axum::debug_handler]
pub async fn get_todo_handler(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    info!("id: {}", id);

    let token = bearer.token();
    let _ = state.auth.verify_user(token)?;

    let todo_res = todolist::get_todo_by_id(id as i64, &state.pool).await?;
    if todo_res.is_none() {
        return Err(AppError::TodoNotExists(format!("todo: {} not found", id)));
    }

    let todo = todo_res.unwrap();
    info!("created todo: {:?}", todo);

    Ok((StatusCode::OK, Json(CreateTodoResp { todo })).into_response())
}

#[axum::debug_handler]
pub async fn update_todo_handler(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(req): Json<UpdateTodosReq>,
) -> Result<impl IntoResponse, AppError> {
    info!("req: {:?}", req);

    let token = bearer.token();
    let user = state.auth.verify_user(token)?;

    let todo_res = todolist::get_todo_by_id(id as i64, &state.pool).await?;
    if todo_res.is_none() {
        return Err(AppError::TodoNotExists(format!("todo: {} not found", id)));
    }
    let todo = todo_res.unwrap();
    if todo.user_id != user.id {
        return Err(AppError::AuthFailed(format!(
            "todo: {} is not belong to user",
            id
        )));
    }

    let todo =
        todolist::update_todo_status(id as i64, req.status.get_status_id(), &state.pool).await?;

    Ok((StatusCode::OK, Json(CreateTodoResp { todo })).into_response())
}

#[axum::debug_handler]
pub async fn list_todo_handler(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    Json(req): Json<ListTodosReq>,
) -> Result<impl IntoResponse, AppError> {
    info!("req: {:?}", req);

    let token = bearer.token();
    let user = state.auth.verify_user(token)?;

    let mut todos = todolist::get_todos_by_user_id(
        GetTodosArgs {
            user_id: user.id,
            offset: req.offset,
            limit: req.limit + 1,
            status: Some(req.status.get_status_id()),
        },
        &state.pool,
    )
    .await?;

    info!("list todos: {:?}", todos);
    let has_more = todos.len() > req.limit as usize;
    if has_more {
        todos.pop();
    }

    Ok((StatusCode::OK, Json(ListTodosResp { todos, has_more })).into_response())
}
