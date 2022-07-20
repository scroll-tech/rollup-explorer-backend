use anyhow::Result;
use dotenv::dotenv;
use rollup_explorer::{cache, db, open_api, Settings};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    Settings::init()?;
    log::debug!("{:?}", Settings::get());

    let mut cache = Arc::new(cache::run()?);
    db::migrator::run().await?;
    open_api::run(cache.clone()).await?;
    Arc::get_mut(&mut cache).unwrap().stop().await
}
