use todolist::{config::AppConfig, run, AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let conf = AppConfig::load("config.toml".to_string())?;
    let state = AppState::new(&conf).await?;
    run(state, &conf).await
}
