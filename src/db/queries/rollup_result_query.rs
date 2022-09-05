use crate::db::models::{RollupResult, RollupStatus};
use crate::db::{table_name, DbPool};
use sqlx::{query_as, query_scalar, Result};
use std::collections::HashMap;

pub async fn fetch_all(db_pool: &DbPool, offset: u64, limit: u64) -> Result<Vec<RollupResult>> {
    let stmt = format!(
        "select
            number, status, rollup_tx_hash, finalize_tx_hash, created_time, updated_time
        from {} order by number desc offset {} limit {}",
        table_name::ROLLUP_RESULT,
        offset,
        limit,
    );
    query_as::<_, RollupResult>(&stmt).fetch_all(db_pool).await
}

pub async fn fetch_ids_by_statuses(
    db_pool: &DbPool,
    statuses: &[RollupStatus],
) -> Result<Vec<i32>> {
    let stmt = format!(
        "select number from {} where status = ANY($1)",
        table_name::ROLLUP_RESULT,
    );
    let statuses: Vec<i32> = statuses.iter().map(Into::into).collect();
    Ok(query_as::<_, (i32,)>(&stmt)
        .bind(&statuses)
        .fetch_all(db_pool)
        .await?
        .into_iter()
        .map(|r| r.0)
        .collect())
}

pub async fn get_by_id(db_pool: &DbPool, id: i32) -> Result<Option<RollupResult>> {
    let stmt = format!(
        "select
            number, status, rollup_tx_hash, finalize_tx_hash, created_time, updated_time
        from {} where number = $1",
        table_name::ROLLUP_RESULT,
    );
    query_as::<_, RollupResult>(&stmt)
        .bind(id)
        .fetch_optional(db_pool)
        .await
}

pub async fn get_last_id_of_statuses(
    db_pool: &DbPool,
    statuses: &[RollupStatus],
) -> Result<Option<i32>> {
    let stmt = format!(
        "select number from {} where status = ANY($1)
        order by number desc limit 1",
        table_name::ROLLUP_RESULT,
    );
    let statuses: Vec<i32> = statuses.iter().map(Into::into).collect();
    query_scalar::<_, i32>(&stmt)
        .bind(&statuses)
        .fetch_optional(db_pool)
        .await
}

pub async fn get_status_max_nums(db_pool: &DbPool) -> Result<HashMap<RollupStatus, i32>> {
    let stmt = format!(
        "select status, max(number) FROM {} group by status",
        table_name::ROLLUP_RESULT,
    );
    query_as::<_, (RollupStatus, i32)>(&stmt)
        .fetch_all(db_pool)
        .await
        .map(|v| v.into_iter().collect())
}

pub async fn get_total(db_pool: &DbPool) -> Result<i32> {
    let stmt = format!(
        "select coalesce(max(number), 0) FROM {}",
        table_name::ROLLUP_RESULT,
    );
    match query_scalar::<_, i32>(&stmt).fetch_one(db_pool).await {
        Ok(max_num) => Ok(max_num),
        Err(error) => Err(error),
    }
}
