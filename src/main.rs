use todolist::{config::AppConfig, run, AppState};
use tracing::info;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let layer = Layer::new().with_filter(tracing::level_filters::LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let conf = AppConfig::load("config.toml".to_string())?;
    let state = AppState::new(&conf).await?;

    info!("start to run server...");
    run(state, &conf).await
}
