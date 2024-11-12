use serde::{Deserialize, Serialize};

use crate::models::{
    todolist::{Todo, TodoStatus},
    user::User,
};

#[derive(Debug, Deserialize)]
pub struct SignupReq {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SignupResp {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginReq {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResp {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoReq {
    pub title: String,
    pub description: String,
    pub status: TodoStatus,
}

#[derive(Debug, Serialize)]
pub struct CreateTodoResp {
    pub todo: Todo,
}

#[derive(Debug, Deserialize)]
pub struct ListTodosReq {
    pub offset: i64,
    pub limit: i64,
    pub status: TodoStatus,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodosReq {
    pub status: TodoStatus,
}

#[derive(Debug, Serialize)]
pub struct ListTodosResp {
    pub todos: Vec<Todo>,
    pub has_more: bool,
}
