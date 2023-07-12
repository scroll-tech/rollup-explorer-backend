use super::chunk_query;
use crate::db::{models::Block, table_name, DbPool};
use sqlx::{query_as, query_scalar, Result};

pub async fn fetch_by_chunk_hash(db_pool: &DbPool, chunk_hash: &str) -> Result<Vec<Block>> {
    let stmt = format!(
        "SELECT
            number,
            tx_num,
            hash,
            chunk_hash,
            block_timestamp
        FROM {}
        WHERE chunk_hash = $1 AND deleted_at IS NULL
        ORDER BY number ASC",
        table_name::BLOCK,
    );

    query_as::<_, Block>(&stmt)
        .bind(chunk_hash)
        .fetch_all(db_pool)
        .await
}

pub async fn fetch_by_num_range(
    db_pool: &DbPool,
    start_num: i64,
    end_num: i64,
) -> Result<Vec<Block>> {
    let stmt = format!(
        "SELECT
            number,
            tx_num,
            hash,
            chunk_hash,
            block_timestamp
        FROM {}
        WHERE number >= $1 AND number <= $2 AND deleted_at IS NULL
        ORDER BY number ASC",
        table_name::BLOCK,
    );

    query_as::<_, Block>(&stmt)
        .bind(start_num)
        .bind(end_num)
        .fetch_all(db_pool)
        .await
}

pub async fn get_batch_hash_by_block_hash(
    db_pool: &DbPool,
    block_hash: &str,
) -> Result<Option<String>> {
    let stmt = format!(
        "SELECT chunk_hash FROM {}
        where LOWER(hash) = LOWER($1) AND deleted_at IS NULL",
        table_name::BLOCK,
    );

    let chunk_hash = query_scalar::<_, String>(&stmt)
        .bind(block_hash)
        .fetch_optional(db_pool)
        .await?;

    Ok(if let Some(chunk_hash) = chunk_hash {
        chunk_query::get_batch_hash_by_chunk_hash(db_pool, &chunk_hash).await?
    } else {
        None
    })
}

pub async fn get_batch_hash_by_number(db_pool: &DbPool, number: i64) -> Result<Option<String>> {
    let stmt = format!(
        "SELECT chunk_hash FROM {}
        where number = $1 AND deleted_at IS NULL",
        table_name::BLOCK,
    );

    let chunk_hash = query_scalar::<_, String>(&stmt)
        .bind(number)
        .fetch_optional(db_pool)
        .await?;

    Ok(if let Some(chunk_hash) = chunk_hash {
        chunk_query::get_batch_hash_by_chunk_hash(db_pool, &chunk_hash).await?
    } else {
        None
    })
}
