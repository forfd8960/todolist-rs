use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::{
    errors::AppError,
    handlers::request::{LoginReq, LoginResp, SignupReq, SignupResp},
    AppState,
};

pub fn get_route(state: AppState) -> Router {
    let route = Router::new()
        .route("/signup", post(signup_handler))
        .route("/signin", post(signin_handler))
        .with_state(state);
    route
}

#[axum::debug_handler]
pub async fn signup_handler(
    State(state): State<AppState>,
    Json(req): Json<SignupReq>,
) -> Result<impl IntoResponse, AppError> {
    println!("{:?}", req);
    Ok((
        StatusCode::OK,
        Json(SignupResp {
            token: "success".to_string(),
        }),
    )
        .into_response())
}

#[axum::debug_handler]
pub async fn signin_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginReq>,
) -> Result<impl IntoResponse, AppError> {
    println!("{:?}", req);
    Ok((
        StatusCode::OK,
        Json(LoginResp {
            token: "success".to_string(),
        }),
    )
        .into_response())
}
