use crate::consts::DEFAULT_CACHE_EXPIRED_SECS;
use anyhow::{anyhow, Result};
use config::{Config, Environment, File};
use serde::Deserialize;
use std::{env, sync::OnceLock};

const DEFAULT_BIND_PORT: &str = "5001";
const DEFAULT_METRICS_BIND_PORT: &str = "6001";
const DEFAULT_MAX_CONNS: u32 = 200;

static SETTINGS: OnceLock<Settings> = OnceLock::new();

#[derive(Debug, Deserialize)]
pub struct Settings {
    /// Internal HTTP bind port (5001 as default)
    pub bind_port: String,
    /// Internal HTTP metrics bind port (6001 as default)
    pub metrics_bind_port: String,
    /// As format of `postgres://USERNAME:PASSWORD@DB_HOST:DB_PORT/DATABASE`
    pub db_url: String,
    /// As format of `HTTP_HOST:HTTP_PORT`
    pub open_api_addr: String,
    ///  Max value of query parameter `per_page` (100 as default)
    pub max_per_page: u64,
    ///  Max DB connections (1000 as default)
    pub max_db_conns: u32,
    ///  Expired cache seconds
    pub cache_expired_secs: u64,
}

impl Settings {
    pub fn init() -> Result<()> {
        let bind_port = env::var("BIND_PORT").unwrap_or_else(|_| DEFAULT_BIND_PORT.into());
        let metrics_bind_port =
            env::var("METRICS_BIND_PORT").unwrap_or_else(|_| DEFAULT_METRICS_BIND_PORT.into());
        let max_db_conns = env::var("MAX_DB_CONNS").map_or_else(
            |_| DEFAULT_MAX_CONNS,
            |conns| conns.parse::<u32>().ok().unwrap_or(DEFAULT_MAX_CONNS),
        );
        let cache_expired_secs = env::var("CACHE_EXPIRED_SECS").map_or_else(
            |_| DEFAULT_CACHE_EXPIRED_SECS,
            |secs| {
                secs.parse::<u64>()
                    .ok()
                    .unwrap_or(DEFAULT_CACHE_EXPIRED_SECS)
            },
        );
        let config = Config::builder()
            .set_default("bind_port", bind_port)?
            .set_default("metrics_bind_port", metrics_bind_port)?
            .set_default("max_per_page", 100)?
            .set_default("max_db_conns", max_db_conns)?
            .set_default("cache_expired_secs", cache_expired_secs)?
            .add_source(File::with_name("config/config.json"))
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
}
