use crate::db::models::RollupResult;
use crate::db::{table_name, DbPool};
use sqlx::{query_as, query_scalar, Result};

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
