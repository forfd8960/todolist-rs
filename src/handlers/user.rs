use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use tracing::info;

use crate::{
    errors::AppError,
    handlers::utils,
    models::user::{self, CreateUser},
    AppState,
};

use super::request::{LoginReq, LoginResp, SignupReq, SignupResp};

#[axum::debug_handler]
pub async fn signup_handler(
    State(state): State<AppState>,
    Json(req): Json<SignupReq>,
) -> Result<impl IntoResponse, AppError> {
    info!("{:?}", req);
    let pwd_hash = utils::gen_password_hash(&req.password)?;

    let create_user = CreateUser {
        username: req.username,
        email: req.email,
        password_hash: pwd_hash,
    };

    let user = user::create_user(&create_user, &state.pool).await?;
    let token = state.auth.sign(user.clone())?;
    info!("create user: {:?}", user);

    Ok((StatusCode::OK, Json(SignupResp { user, token })).into_response())
}

#[axum::debug_handler]
pub async fn signin_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginReq>,
) -> Result<impl IntoResponse, AppError> {
    println!("{:?}", req);

    let user_res = user::get_user_by_email(&req.email, &state.pool).await?;
    if user_res.is_none() {
        return Err(AppError::UserNotFound(format!(
            "user {} not found",
            &req.email
        )));
    }

    let user = user_res.unwrap();
    let verified = utils::verify_password(&req.password, &user.password_hash.clone().unwrap())?;
    if !verified {
        return Err(AppError::AuthFailed(format!(
            "user: {} password is not correct",
            &req.email
        )));
    }

    let token = state.auth.sign(user)?;
    Ok((StatusCode::OK, Json(LoginResp { token })).into_response())
}
