use anyhow::Result;
use rollup_explorer::{db, open_api, Settings};

#[tokio::main]
async fn main() -> Result<()> {
    Settings::init()?;
    log::debug!("{:?}", Settings::get());

    db::migrator::run().await?;
    open_api::run().await
}
