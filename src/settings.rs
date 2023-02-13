use anyhow::{anyhow, Result};
use config::{Config, Environment, File};
use serde::Deserialize;
use std::env;
use std::sync::OnceLock;

const DEFAULT_BIND_PORT: &str = "5001";

static SETTINGS: OnceLock<Settings> = OnceLock::new();

#[derive(Debug, Deserialize)]
pub struct Settings {
    /// Internal HTTP bind port (5001 as default)
    pub bind_port: String,
    /// As format of `postgres://USERNAME:PASSWORD@DB_HOST:DB_PORT/DATABASE`
    pub db_url: String,
    /// Base path for the server to serve requests on ('/' as default)
    pub api_base_path: String,
    /// As format of `HTTP_HOST:HTTP_PORT`
    pub open_api_addr: String,
    /// `development` or `production`
    run_mode: String,
    ///  Max value of query parameter `per_page` (100 as default)
    pub max_per_page: u64,
}

impl Settings {
    pub fn init() -> Result<()> {
        let bind_port = env::var("BIND_PORT").unwrap_or_else(|_| DEFAULT_BIND_PORT.into());
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        let config = Config::builder()
            .set_default("bind_port", bind_port)?
            .set_default("api_base_path", "/")?
            .set_default("run_mode", run_mode.clone())?
            .set_default("max_per_page", 100)?
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(Environment::default())
            .build()?;

        let settings: Settings = config.try_deserialize()?;
        SETTINGS
            .set(settings)
            .map_err(|s| anyhow!("Wrong settings: {:?}", s))?;

        Ok(())
    }

    pub fn get() -> &'static Self {
        SETTINGS.get().unwrap()
    }

    pub fn is_dev(&self) -> bool {
        self.run_mode == "development"
    }

    pub fn is_prod(&self) -> bool {
        self.run_mode == "production"
    }
}
