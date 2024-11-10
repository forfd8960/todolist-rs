use auth::Authorization;
use config::AppConfig;
use errors::AppError;
use route::get_route;
use sqlx::PgPool;
use std::{ops::Deref, sync::Arc};
use tokio::net::TcpListener;

pub mod auth;
pub mod config;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod route;

pub async fn run(state: AppState, conf: &AppConfig) -> anyhow::Result<()> {
    println!("app-todolist start...");

    let addr = format!("0.0.0.0:{}", conf.server.port);
    let listener = TcpListener::bind(&addr).await?;

    println!("app-todolist listen on: {}", addr);
    let app = get_route(state);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[derive(Debug, Clone)]
pub struct AppState {
    state: Arc<InnerAppState>,
}

impl AppState {
    pub async fn new(conf: &AppConfig) -> Result<Self, AppError> {
        let pool = PgPool::connect(&conf.server.db_url).await?;
        Ok(Self {
            state: Arc::new(InnerAppState {
                pool: pool,
                auth: Authorization {},
            }),
        })
    }
}

#[derive(Debug)]
pub struct InnerAppState {
    pub(crate) pool: PgPool,
    pub(crate) auth: Authorization,
}

impl Deref for AppState {
    type Target = InnerAppState;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
