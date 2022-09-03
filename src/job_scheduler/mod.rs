use crate::cache::Cache;
use crate::db::DbPool;
use crate::Settings;
use anyhow::Result;
use std::sync::Arc;
use tokio_cron_scheduler::JobScheduler;

mod last_finalized_monitor;
mod precommitted_monitor;

use last_finalized_monitor::*;
use precommitted_monitor::*;

pub async fn start(cache: Arc<Cache>) -> Result<JobScheduler> {
    let settings = Settings::get();
    let db_pool = Arc::new(DbPool::connect(settings.db_url.as_str()).await?);

    let scheduler = JobScheduler::new().await?;

    scheduler
        .add(last_finalized_monitor_job(cache.clone(), db_pool.clone())?)
        .await?;
    scheduler
        .add(precommitted_monitor_job(cache, db_pool)?)
        .await?;

    scheduler.shutdown_on_ctrl_c();
    scheduler.start().await?;

    Ok(scheduler)
}

pub async fn stop(mut job_scheduler: JobScheduler) -> Result<()> {
    Ok(job_scheduler.shutdown().await?)
}
