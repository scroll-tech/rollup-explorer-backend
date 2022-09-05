use crate::cache::Cache;
use crate::db::models::RollupStatus;
use crate::db::{rollup_result_query, DbPool};
use crate::{slack, Settings};
use anyhow::Result;
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio_cron_scheduler::Job;

const CACHE_KEY: &str = "scheduled-job-precommitted-ids";

pub fn precommitted_monitor_jobs(cache: Arc<Cache>, db_pool: Arc<DbPool>) -> Result<Vec<Job>> {
    let job_interval_secs = Settings::get().monitor_precommitted_interval_secs;
    let cache_clone = cache.clone();
    let db_pool_clone = db_pool.clone();
    let one_shot_job = Job::new_one_shot_async(Duration::from_secs(1), move |_, _| {
        new_task(
            job_interval_secs,
            cache_clone.clone(),
            db_pool_clone.clone(),
        )
    })?;
    let repeated_job =
        Job::new_repeated_async(Duration::from_secs(job_interval_secs), move |_, _| {
            new_task(job_interval_secs, cache.clone(), db_pool.clone())
        })?;

    Ok(vec![one_shot_job, repeated_job])
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

fn new_task(
    task_expired_secs: u64,
    cache: Arc<Cache>,
    db_pool: Arc<DbPool>,
) -> Pin<Box<impl Future<Output = ()>>> {
    Box::pin(async move {
        log::info!("Running precommitted_monitor_job");

        // Get current `precommitted` IDs from DB.
        let new_ids = get_ids_from_db(db_pool)
            .await
            .expect("Failed to get precommitted IDs from DB in precommitted_monitor_job");
        log::debug!("precommitted_monitor_job: new_ids = {new_ids:?}");

        // Get previous `precommitted` IDs from cache.
        let old_ids = get_ids_from_cache(cache.clone()).await;
        log::debug!("precommitted_monitor_job: old_ids = {old_ids:?}");

        let delayed_ids: Vec<_> = old_ids.intersection(&new_ids).collect();
        log::debug!("precommitted_monitor_job: deplayed_ids = {delayed_ids:?}");
        // Notify deplayed `precommitted` IDs to a Slack channel.
        if !delayed_ids.is_empty() {
            let msg = format!(
                "`Precommitted` blocks stayed for more than `{task_expired_secs}`s: \
                        committed_ids = `{delayed_ids:?}`.",
            );
            slack::notify(&msg)
                .await
                .expect("Failed to notify to Slack channel in precommitted_monitor_job");
        }

        let cache_expired_secs = task_expired_secs * 3 / 2;
        set_ids_to_cache(cache, new_ids, cache_expired_secs)
            .await
            .expect("Failed to set precommitted IDs to cache in precommitted_monitor_job");
    })
}

async fn set_ids_to_cache(cache: Arc<Cache>, ids: HashSet<i32>, expired_secs: u64) -> Result<()> {
    cache.set(CACHE_KEY, Arc::new(ids), expired_secs).await
}
