use axum::{routing::post, Router};

use crate::{
    handlers::user::{signin_handler, signup_handler},
    AppState,
};

pub fn get_route(state: AppState) -> Router {
    let route = Router::new()
        .route("/signup", post(signup_handler))
        .route("/signin", post(signin_handler))
        .with_state(state);
    route
}
