use crate::db::{models::Batch, table_name, DbPool, RollupStatusType};
use sqlx::{query_as, query_scalar, Result};
use std::collections::HashMap;

pub async fn fetch_all(db_pool: &DbPool, offset: u64, limit: u64) -> Result<Vec<Batch>> {
    let stmt = format!(
        "SELECT
            hash,
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
        table_name::BATCH,
        offset,
        limit,
    );
    query_as::<_, Batch>(&stmt).fetch_all(db_pool).await
}

pub async fn fetch_one(db_pool: &DbPool, index: i64) -> Result<Option<Batch>> {
    let stmt = format!(
        "SELECT
            hash,
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
        FROM {} where index = $1",
        table_name::BATCH,
    );
    query_as::<_, Batch>(&stmt)
        .bind(index)
        .fetch_optional(db_pool)
        .await
}

pub async fn get_hash_by_index(db_pool: &DbPool, index: i64) -> Result<Option<String>> {
    let stmt = format!("SELECT hash FROM {} where index = $1", table_name::BATCH,);
    query_scalar::<_, String>(&stmt)
        .bind(index)
        .fetch_optional(db_pool)
        .await
}

pub async fn get_index_by_hash(db_pool: &DbPool, hash: &str) -> Result<Option<i64>> {
    let stmt = format!("SELECT index FROM {} where hash = $1", table_name::BATCH,);
    query_scalar::<_, i64>(&stmt)
        .bind(hash)
        .fetch_optional(db_pool)
        .await
}

pub async fn get_total(db_pool: &DbPool) -> Result<i64> {
    let stmt = format!("SELECT COALESCE(MAX(index), 0) FROM {}", table_name::BATCH,);
    query_scalar::<_, i64>(&stmt).fetch_one(db_pool).await
}

pub async fn get_max_status_indexes(db_pool: &DbPool) -> Result<HashMap<RollupStatusType, i64>> {
    let stmt = format!(
        "select rollup_status, max(index) FROM {} group by rollup_status",
        table_name::BATCH,
    );
    query_as::<_, (RollupStatusType, i64)>(&stmt)
        .fetch_all(db_pool)
        .await
        .map(|v| v.into_iter().collect())
}
