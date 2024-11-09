use auth::Authorization;
use sqlx::PgPool;
use std::{ops::Deref, sync::Arc};

pub mod auth;
pub mod config;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod route;

#[derive(Debug)]
pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        println!("app-todolist start...")
    }
}

#[derive(Debug)]
pub struct AppState {
    state: Arc<InnerAppState>,
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
