use std::sync::Arc;

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
pub struct InnerAppState {}
