use crate::cache::Cache;
use crate::db::models::RollupStatus;
use crate::db::{rollup_result_query, DbPool};
use crate::{slack, Settings};
use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio_cron_scheduler::Job;

const CACHE_KEY: &str = "scheduled-job-last-finalized-next-committed-id";

pub fn last_finalized_monitor_job(cache: Arc<Cache>, db_pool: Arc<DbPool>) -> Result<Job> {
    let job_interval_secs = Settings::get().monitor_last_finalized_interval_secs;
    Ok(Job::new_repeated_async(
        Duration::from_secs(job_interval_secs),
        move |_, _| {
            let cache = cache.clone();
            let db_pool = db_pool.clone();
            Box::pin(async move {
                log::info!("Running last_finalized_monitor_job");

                // Get last `finalized` ID from DB.
                let last_finalized_id =
                    get_last_finalized_id_from_db(db_pool.clone()).await.expect(
                        "Failed to get last finalized ID from DB in last_finalized_monitor_job",
                    );
                if last_finalized_id.is_none() {
                    return;
                }
                let last_finalized_id = last_finalized_id.unwrap();

                // Get previous `next-committed` ID from cache.
                let old_committed_id = get_committed_id_from_cache(cache.clone()).await;
                let new_committed_id =
                    if let Some(old_committed_id) = old_committed_id
                        && old_committed_id == last_finalized_id + 1 {
                        // Notify this delayed `committed` ID to a Slack channel.
                        let msg = format!(
                            "Last 'finalized' block's next 'committed' block stayed for more than \
                            {job_interval_secs}s: last_finalized_id = '{last_finalized_id}', \
                            next_committed_id = '{old_committed_id}'.",
                        );
                        slack::notify(&msg)
                            .await
                            .expect("Failed to notify to Slack channel in last_finalized_monitor_job");

                        // Return previous `committed` ID, and set it to cache again to avoid expired.
                        Some(old_committed_id)
                    } else {
                        // Get new `next-committed` ID from DB.
                        get_committed_id_from_db(db_pool, last_finalized_id + 1)
                            .await
                            .expect("Failed to get next committed ID from DB in last_finalized_monitor_job")
                    };

                if let Some(new_committed_id) = new_committed_id {
                    let cache_expired_secs = job_interval_secs * 3 / 2;
                    set_committed_id_to_cache(cache, new_committed_id, cache_expired_secs)
                        .await
                        .expect("Failed to set next committed ID to cache in last_finalized_monitor_job");
                }
            })
        },
    )?)
}

async fn get_committed_id_from_cache(cache: Arc<Cache>) -> Option<i32> {
    cache
        .get(CACHE_KEY)
        .await
        .ok()
        .flatten()
        .and_then(|any| any.downcast_ref::<i32>().cloned())
}

async fn get_committed_id_from_db(db_pool: Arc<DbPool>, id: i32) -> Result<Option<i32>> {
    let rollup_result = rollup_result_query::get_by_id(&db_pool, id).await?;
    if let Some(rollup_result) = rollup_result &&
        RollupStatus::map_from_str("committed").contains(&rollup_result.status) {
        return Ok(Some(rollup_result.number));
    }

    Ok(None)
}

async fn get_last_finalized_id_from_db(db_pool: Arc<DbPool>) -> Result<Option<i32>> {
    let statuses = RollupStatus::map_from_str("finalized");
    Ok(rollup_result_query::get_last_id_of_statuses(&db_pool, &statuses).await?)
}

async fn set_committed_id_to_cache(cache: Arc<Cache>, id: i32, expired_secs: u64) -> Result<()> {
    cache.set(CACHE_KEY, Arc::new(id), expired_secs).await
}
