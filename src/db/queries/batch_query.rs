use super::chunk_query;
use crate::db::{models::Batch, table_name, DbPool, RollupStatusType};
use sqlx::{query_as, query_scalar, Result};
use std::collections::HashMap;

pub async fn fetch_all(db_pool: &DbPool, offset: u64, limit: u64) -> Result<Vec<Batch>> {
    let stmt = format!(
        "SELECT
            hash,
            index,
            start_chunk_index,
            end_chunk_index,
            rollup_status,
            commit_tx_hash,
            finalize_tx_hash,
            created_at,
            committed_at,
            finalized_at
        FROM {}
        WHERE deleted_at IS NULL
        ORDER BY index DESC OFFSET {} LIMIT {}",
        table_name::BATCH,
        offset,
        limit,
    );

    let mut batches = query_as::<_, Batch>(&stmt).fetch_all(db_pool).await?;
    for batch in batches.iter_mut() {
        complete_batch(db_pool, batch).await?;
    }

    Ok(batches)
}

pub async fn fetch_one(db_pool: &DbPool, index: i64) -> Result<Option<Batch>> {
    let stmt = format!(
        "SELECT
            hash,
            index,
            start_chunk_index,
            end_chunk_index,
            rollup_status,
            commit_tx_hash,
            finalize_tx_hash,
            created_at,
            committed_at,
            finalized_at
        FROM {}
        WHERE index = $1 AND deleted_at IS NULL",
        table_name::BATCH,
    );

    let batch = query_as::<_, Batch>(&stmt)
        .bind(index)
        .fetch_optional(db_pool)
        .await?;
    Ok(if let Some(mut batch) = batch {
        complete_batch(db_pool, &mut batch).await?;
        Some(batch)
    } else {
        None
    })
}

pub async fn get_hash_by_index(db_pool: &DbPool, index: i64) -> Result<Option<String>> {
    let stmt = format!(
        "SELECT hash FROM {}
        WHERE index = $1 AND deleted_at IS NULL",
        table_name::BATCH,
    );

    query_scalar::<_, String>(&stmt)
        .bind(index)
        .fetch_optional(db_pool)
        .await
}

pub async fn get_index_by_hash(db_pool: &DbPool, hash: &str) -> Result<Option<i64>> {
    let stmt = format!(
        "SELECT index FROM {}
        WHERE hash = $1 AND deleted_at IS NULL",
        table_name::BATCH,
    );

    query_scalar::<_, i64>(&stmt)
        .bind(hash)
        .fetch_optional(db_pool)
        .await
}

pub async fn get_total(db_pool: &DbPool) -> Result<i64> {
    let stmt = format!(
        "SELECT COALESCE(MAX(index), 0) FROM {}
        WHERE deleted_at IS NULL",
        table_name::BATCH,
    );

    query_scalar::<_, i64>(&stmt).fetch_one(db_pool).await
}

pub async fn get_max_status_indexes(db_pool: &DbPool) -> Result<HashMap<RollupStatusType, i64>> {
    let stmt = format!(
        "SELECT rollup_status, MAX(index) FROM {}
        WHERE deleted_at IS NULL
        GROUP BY rollup_status",
        table_name::BATCH,
    );

    query_as::<_, (RollupStatusType, i64)>(&stmt)
        .fetch_all(db_pool)
        .await
        .map(|v| v.into_iter().collect())
}

async fn complete_batch(db_pool: &DbPool, batch: &mut Batch) -> Result<()> {
    batch.start_block_number =
        chunk_query::get_start_block_number_by_index(db_pool, batch.start_chunk_index).await?;
    batch.end_block_number =
        chunk_query::get_end_block_number_by_index(db_pool, batch.end_chunk_index).await?;
    batch.total_tx_num = Some(
        chunk_query::get_total_tx_num_by_index_range(
            db_pool,
            batch.start_chunk_index,
            batch.end_chunk_index,
        )
        .await?,
    );

    Ok(())
}
