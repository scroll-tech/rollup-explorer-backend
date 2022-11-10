use crate::db::models::BlockResult;
use crate::db::{table_name, DbPool};
use sqlx::{query_as, Result};

pub async fn fetch_all(db_pool: &DbPool, batch_id: &str) -> Result<Vec<BlockResult>> {
    let stmt = format!(
        "SELECT
            number,
            tx_num,
            hash,
            batch_id,
            block_timestamp
        FROM {} WHERE batch_id = $1 ORDER BY number ASC",
        table_name::BLOCK_RESULT,
    );
    query_as::<_, BlockResult>(&stmt)
        .bind(batch_id)
        .fetch_all(db_pool)
        .await
}
