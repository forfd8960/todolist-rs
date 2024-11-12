use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::{
        todolist::{create_todo_handler, get_todo_handler, list_todo_handler, update_todo_handler},
        user::{signin_handler, signup_handler},
    },
    AppState,
};

pub fn get_route(state: AppState) -> Router {
    let route = Router::new()
        .route("/todos", post(create_todo_handler).get(list_todo_handler))
        .route("/todos/:id", get(get_todo_handler).put(update_todo_handler))
        .route("/signup", post(signup_handler))
        .route("/signin", post(signin_handler))
        .with_state(state);
    route
}
