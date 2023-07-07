use crate::db::{models::Block, table_name, DbPool};
use sqlx::{query_as, query_scalar, Result};

pub async fn fetch_all(db_pool: &DbPool, batch_hash: &str) -> Result<Vec<Block>> {
    let stmt = format!(
        "SELECT
            number,
            tx_num,
            hash,
            batch_hash,
            block_timestamp
        FROM {} WHERE batch_hash = $1 ORDER BY number ASC",
        table_name::BLOCK,
    );
    query_as::<_, Block>(&stmt)
        .bind(batch_hash)
        .fetch_all(db_pool)
        .await
}

pub async fn get_batch_hash_by_trace_hash(
    db_pool: &DbPool,
    trace_hash: &str,
) -> Result<Option<String>> {
    let stmt = format!(
        "SELECT batch_hash FROM {} where LOWER(hash) = LOWER($1)",
        table_name::BLOCK,
    );
    query_scalar::<_, String>(&stmt)
        .bind(trace_hash)
        .fetch_optional(db_pool)
        .await
}

pub async fn get_batch_hash_by_number(db_pool: &DbPool, number: i64) -> Result<Option<String>> {
    let stmt = format!(
        "SELECT batch_hash FROM {} where number = $1",
        table_name::BLOCK,
    );
    query_scalar::<_, String>(&stmt)
        .bind(number)
        .fetch_optional(db_pool)
        .await
}
