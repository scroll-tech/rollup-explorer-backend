use anyhow::{anyhow, Error, Result};
use config::{Config, Environment, File};
use serde::Deserialize;
use std::env;
use std::sync::OnceLock;

const DEFAULT_MONITOR_PRECOMMITTED_INTERVAL_SECS: u64 = 60 * 2; // 2-minutes
const DEFAULT_MONITOR_LAST_FINALIZED_INTERVAL_SECS: u64 = 60 * 30; // 30-minutes

static SETTINGS: OnceLock<Settings> = OnceLock::new();

#[derive(Debug, Deserialize)]
pub struct Settings {
    /// Interval seconds of repeated last `finalized` monitor job
    pub monitor_last_finalized_interval_secs: u64,
    /// Interval seconds of repeated `precommitted` monitor job
    pub monitor_precommitted_interval_secs: u64,
    /// Slack notify URL
    pub slack_notify_url: Option<String>,
    /// As format of `postgres://USERNAME:PASSWORD@DB_HOST:DB_PORT/DATABASE`
    pub db_url: String,
    /// As format of `HTTP_HOST:HTTP_PORT`
    pub open_api_addr: String,
    /// `development` or `production`
    run_mode: String,
}

impl Settings {
    pub fn init() -> Result<()> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        let slack_notify_url = env::var_os("SLACK_NOTIFY_URL").and_then(|os| os.into_string().ok());
        let config = Config::builder()
            .set_default(
                "monitor_last_finalized_interval_secs",
                get_monitor_last_finalized_interval_secs(),
            )?
            .set_default(
                "monitor_precommitted_interval_secs",
                get_monitor_precommitted_interval_secs(),
            )?
            .set_default("run_mode", run_mode.clone())?
            .set_default("slack_notify_url", slack_notify_url)?
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

fn get_monitor_last_finalized_interval_secs() -> u64 {
    env::var("MONITOR_LAST_FINALIZED_INTERVAL_SECS")
        .map_err(Error::msg)
        .and_then(|s| s.parse::<u64>().map_err(Error::msg))
        .unwrap_or(DEFAULT_MONITOR_LAST_FINALIZED_INTERVAL_SECS)
}

fn get_monitor_precommitted_interval_secs() -> u64 {
    env::var("MONITOR_PRECOMMITTED_INTERVAL_SECS")
        .map_err(Error::msg)
        .and_then(|s| s.parse::<u64>().map_err(Error::msg))
        .unwrap_or(DEFAULT_MONITOR_PRECOMMITTED_INTERVAL_SECS)
}
