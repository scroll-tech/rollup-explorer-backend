use crate::cache::Cache;
use crate::db::models::RollupStatus;
use crate::db::{rollup_result_query, DbPool};
use crate::Settings;
use anyhow::Result;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio_cron_scheduler::Job;

const CACHE_KEY: &str = "scheduled-job-precommitted-ids";

pub fn precommitted_monitor_job(cache: Arc<Cache>, db_pool: Arc<DbPool>) -> Result<Job> {
    let job_interval_secs = Settings::get().monitor_precommitted_interval_secs;
    Ok(Job::new_repeated_async(
        Duration::from_secs(job_interval_secs),
        move |_, _| {
            let cache = cache.clone();
            let db_pool = db_pool.clone();
            Box::pin(async move {
                log::info!("Running precommitted_monitor_job");

                // Get current `precommitted` IDs from DB.
                let new_ids = get_ids_from_db(db_pool)
                    .await
                    .expect("Failed to get precommitted IDs from DB in precommitted_monitor_job");

                // Get previous `precommitted` IDs from cache.
                let old_ids = get_ids_from_cache(cache.clone()).await;

                let _delayed_ids: Vec<_> = old_ids.intersection(&new_ids).collect();
                // TODO: Notify deplayed IDs to a Slack channel.

                let cache_expired_secs = job_interval_secs * 3 / 2;
                set_ids_to_cache(cache, new_ids, cache_expired_secs)
                    .await
                    .expect("Failed to set precommitted IDs to cache in precommitted_monitor_job");
            })
        },
    )?)
}

async fn get_ids_from_cache(cache: Arc<Cache>) -> HashSet<i32> {
    cache
        .get(CACHE_KEY)
        .await
        .ok()
        .flatten()
        .and_then(|any| any.downcast_ref::<HashSet<i32>>().cloned())
        .unwrap_or_default()
}

async fn get_ids_from_db(db_pool: Arc<DbPool>) -> Result<HashSet<i32>> {
    let statuses = RollupStatus::map_from_str("precommitted");
    let ids = rollup_result_query::fetch_ids_by_statuses(&db_pool, &statuses).await?;
    Ok(HashSet::from_iter(ids.iter().cloned()))
}

async fn set_ids_to_cache(cache: Arc<Cache>, ids: HashSet<i32>, expired_secs: u64) -> Result<()> {
    cache.set(CACHE_KEY, Arc::new(ids), expired_secs).await
}
