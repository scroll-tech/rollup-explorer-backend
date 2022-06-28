use anyhow::{anyhow, Result};
use config::{Config, File};
use serde::Deserialize;
use std::env;
use std::lazy::SyncOnceCell;

static SETTINGS: SyncOnceCell<Settings> = SyncOnceCell::new();

#[derive(Debug, Deserialize)]
pub struct Settings {
    /// As format of `postgres://USERNAME:PASSWORD@DB_HOST:DB_PORT/DATABASE`
    pub db_url: String,
    /// As format of `HTTP_HOST:HTTP_PORT`
    pub open_api_addr: String,
}

impl Settings {
    pub fn init() -> Result<()> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .build()?;

        let settings = config.try_deserialize()?;
        SETTINGS
            .set(settings)
            .map_err(|s| anyhow!("Wrong settings: {:?}", s))?;

        Ok(())
    }

    pub fn get() -> &'static Self {
        SETTINGS.get().unwrap()
    }
}
