use anyhow::Result;
use dotenvy::dotenv;
use rollup_explorer::{cache, open_api, Settings};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .init();

    Settings::init()?;
    log::debug!("{:?}", Settings::get());

    let mut cache = Arc::new(cache::run()?);
    open_api::run(cache.clone()).await?;
    Arc::get_mut(&mut cache).unwrap().stop().await
}
