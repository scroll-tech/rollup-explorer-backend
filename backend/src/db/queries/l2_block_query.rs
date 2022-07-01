use crate::db::models::L2Block;
use crate::db::{table_name, DbPool};
use sqlx::Error::RowNotFound;
use sqlx::{query_as, query_scalar, Result};

pub async fn get_total(db_pool: &DbPool) -> Result<i64> {
    let stmt = format!(
        "select id from {} order by id desc limit 1",
        table_name::L2_BLOCK
    );
    match query_scalar::<_, i64>(&stmt).fetch_one(db_pool).await {
        Ok(max_id) => Ok(max_id),
        Err(RowNotFound) => Ok(0),
        Err(error) => Err(error),
    }
}

pub async fn fetch_all(db_pool: &DbPool, offset: u64, limit: u64) -> Result<Vec<L2Block>> {
    let stmt = format!(
        "select
            id, status, header_hash, l1_tx_hash, tx_num, timestamp, created_at, updated_at
        from {} order by id desc offset {} limit {}",
        table_name::L2_BLOCK,
        offset,
        limit,
    );
    query_as::<_, L2Block>(&stmt).fetch_all(db_pool).await
}
