use anyhow::Result;
use dotenv::dotenv;
use rollup_explorer::{cache, job_scheduler, open_api, Settings};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    Settings::init()?;
    log::debug!("{:?}", Settings::get());

    let mut cache = Arc::new(cache::run()?);

    log::debug!("Start job scheduler");
    let job_scheduler = job_scheduler::start(cache.clone()).await?;

    log::debug!("Start Open API");
    open_api::run(cache.clone()).await?;
    Arc::get_mut(&mut cache).unwrap().stop().await?;

    log::debug!("Stop job scheduler");
    job_scheduler::stop(job_scheduler).await
}
