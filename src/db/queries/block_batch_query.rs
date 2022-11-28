use crate::db::models::BlockBatch;
use crate::db::{table_name, DbPool, RollupStatusType};
use sqlx::{query_as, query_scalar, Result};
use std::collections::HashMap;

pub async fn fetch_all(db_pool: &DbPool, offset: u64, limit: u64) -> Result<Vec<BlockBatch>> {
    let stmt = format!(
        "SELECT
            id,
            index,
            start_block_number,
            end_block_number,
            total_tx_num,
            rollup_status,
            commit_tx_hash,
            finalize_tx_hash,
            created_at,
            committed_at,
            finalized_at
        FROM {} ORDER BY index DESC OFFSET {} LIMIT {}",
        table_name::BLOCK_BATCH,
        offset,
        limit,
    );
    query_as::<_, BlockBatch>(&stmt).fetch_all(db_pool).await
}

pub async fn fetch_one(db_pool: &DbPool, batch_id: &str) -> Result<Option<BlockBatch>> {
    let stmt = format!(
        "SELECT
            id,
            index,
            start_block_number,
            end_block_number,
            total_tx_num,
            rollup_status,
            commit_tx_hash,
            finalize_tx_hash,
            created_at,
            committed_at,
            finalized_at
        FROM {} where id = $1",
        table_name::BLOCK_BATCH,
    );
    query_as::<_, BlockBatch>(&stmt)
        .bind(batch_id)
        .fetch_optional(db_pool)
        .await
}

pub async fn get_total(db_pool: &DbPool) -> Result<i64> {
    let stmt = format!(
        "SELECT COALESCE(MAX(index), 0) FROM {}",
        table_name::BLOCK_BATCH,
    );
    match query_scalar::<_, i64>(&stmt).fetch_one(db_pool).await {
        Ok(max_num) => Ok(max_num),
        Err(error) => Err(error),
    }
}

pub async fn get_max_status_indexes(db_pool: &DbPool) -> Result<HashMap<RollupStatusType, i64>> {
    let stmt = format!(
        "select rollup_status, max(index) FROM {} group by rollup_status",
        table_name::BLOCK_BATCH,
    );
    query_as::<_, (RollupStatusType, i64)>(&stmt)
        .fetch_all(db_pool)
        .await
        .map(|v| v.into_iter().collect())
}