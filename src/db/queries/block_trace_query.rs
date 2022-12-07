use crate::db::models::BlockTrace;
use crate::db::{table_name, DbPool};
use sqlx::{query_as, query_scalar, Result};

pub async fn fetch_all(db_pool: &DbPool, batch_id: &str) -> Result<Vec<BlockTrace>> {
    let stmt = format!(
        "SELECT
            number,
            tx_num,
            hash,
            batch_id,
            block_timestamp
        FROM {} WHERE batch_id = $1 ORDER BY number ASC",
        table_name::BLOCK_TRACE,
    );
    query_as::<_, BlockTrace>(&stmt)
        .bind(batch_id)
        .fetch_all(db_pool)
        .await
}

pub async fn get_batch_id_by_hash(db_pool: &DbPool, hash: &str) -> Result<Option<String>> {
    let stmt = format!(
        "SELECT batch_id FROM {} where LOWER(hash) = LOWER($1)",
        table_name::BLOCK_TRACE,
    );
    query_scalar::<_, String>(&stmt)
        .bind(hash)
        .fetch_optional(db_pool)
        .await
}

pub async fn get_batch_id_by_number(db_pool: &DbPool, number: i64) -> Result<Option<String>> {
    let stmt = format!(
        "SELECT batch_id FROM {} where number = $1",
        table_name::BLOCK_TRACE,
    );
    query_scalar::<_, String>(&stmt)
        .bind(number)
        .fetch_optional(db_pool)
        .await
}
