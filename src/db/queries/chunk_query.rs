use crate::db::{models::Chunk, table_name, DbPool};
use sqlx::{query_as, query_scalar, Result};

pub async fn fetch_by_batch_hash(db_pool: &DbPool, batch_hash: &str) -> Result<Vec<Chunk>> {
    let stmt = format!(
        "SELECT
            index,
            start_block_number,
            end_block_number,
            (total_l1_messages_popped_in_chunk + total_l2_tx_num) AS total_tx_num,
            hash,
            batch_hash,
            created_at
        FROM {}
        WHERE batch_hash = $1 AND deleted_at IS NULL
        ORDER BY index ASC",
        table_name::CHUNK,
    );

    query_as::<_, Chunk>(&stmt)
        .bind(batch_hash)
        .fetch_all(db_pool)
        .await
}

pub async fn get_batch_hash_by_chunk_hash(
    db_pool: &DbPool,
    chunk_hash: &str,
) -> Result<Option<String>> {
    let stmt = format!(
        "SELECT batch_hash FROM {}
        WHERE LOWER(hash) = LOWER($1) AND deleted_at IS NULL",
        table_name::CHUNK,
    );

    query_scalar::<_, String>(&stmt)
        .bind(chunk_hash)
        .fetch_optional(db_pool)
        .await
}

pub async fn get_block_num_range_by_batch_hash(
    db_pool: &DbPool,
    batch_hash: &str,
) -> Result<(i64, i64)> {
    let stmt = format!(
        "SELECT
            COALESCE(MIN(start_block_number), 0),
            COALESCE(MAX(end_block_number), 0)
        FROM {}
        WHERE batch_hash = $1 AND deleted_at IS NULL",
        table_name::CHUNK,
    );

    query_as::<_, (i64, i64)>(&stmt)
        .bind(batch_hash)
        .fetch_one(db_pool)
        .await
}

pub async fn get_end_block_number_by_index(
    db_pool: &DbPool,
    chunk_index: i64,
) -> Result<Option<i64>> {
    let stmt = format!(
        "SELECT end_block_number FROM {}
        WHERE index = $1",
        table_name::CHUNK
    );
    query_scalar::<_, i64>(&stmt)
        .bind(chunk_index)
        .fetch_optional(db_pool)
        .await
}

pub async fn get_hash_by_index(db_pool: &DbPool, index: i64) -> Result<Option<String>> {
    let stmt = format!(
        "SELECT hash FROM {}
        WHERE index = $1 AND deleted_at IS NULL",
        table_name::CHUNK,
    );

    query_scalar::<_, String>(&stmt)
        .bind(index)
        .fetch_optional(db_pool)
        .await
}

pub async fn get_start_block_number_by_index(
    db_pool: &DbPool,
    chunk_index: i64,
) -> Result<Option<i64>> {
    let stmt = format!(
        "SELECT start_block_number FROM {}
        where index = $1",
        table_name::CHUNK,
    );

    query_scalar::<_, i64>(&stmt)
        .bind(chunk_index)
        .fetch_optional(db_pool)
        .await
}

pub async fn get_total_tx_num_by_index_range(
    db_pool: &DbPool,
    start_chunk_index: i64,
    end_chunk_index: i64,
) -> Result<i64> {
    let stmt = format!(
        "SELECT
            COALESCE(
                SUM(total_l1_messages_popped_in_chunk + total_l2_tx_num),
                0
            ) FROM {}
            WHERE index >= $1 AND index <= $2 AND deleted_at IS NULL",
        table_name::CHUNK,
    );

    query_scalar::<_, i64>(&stmt)
        .bind(start_chunk_index)
        .bind(end_chunk_index)
        .fetch_one(db_pool)
        .await
}
