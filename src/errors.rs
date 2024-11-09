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
}
