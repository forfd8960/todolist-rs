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
    handlers::request::CreateTodoResp,
    models::todolist::{self, CreateTodo},
    AppState,
};

use super::request::CreateTodoReq;

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
