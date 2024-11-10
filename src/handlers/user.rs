use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{errors::AppError, AppState};

use super::request::{LoginReq, LoginResp, SignupReq, SignupResp};

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
