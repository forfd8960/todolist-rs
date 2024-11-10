use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("user email already been used: {0}")]
    EmailAlreadyExists(String),

    #[error("user auth failed: {0}")]
    AuthFailed(String),

    #[error("load config err: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("db error: {0}")]
    DBError(#[from] sqlx::Error),

    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("todo not exists: {0}")]
    TodoNotExists(String),

    #[error("anyhow error: {0}")]
    AnyhowError(#[from] anyhow::Error),

    #[error("pwd hash error: {0}")]
    PasswordHashError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            Self::EmailAlreadyExists(_) => StatusCode::BAD_REQUEST,
            Self::AuthFailed(_) => StatusCode::UNAUTHORIZED,
            Self::TomlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TodoNotExists(_) => StatusCode::NOT_FOUND,
            Self::AnyhowError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PasswordHashError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, format!("{}", self)).into_response()
    }
}
