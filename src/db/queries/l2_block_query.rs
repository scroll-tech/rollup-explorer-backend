use crate::db::models::L2Block;
use crate::db::{table_name, DbPool};
use rust_decimal::Decimal;
use sqlx::Error::RowNotFound;
use sqlx::{query_as, query_scalar, Result};

pub async fn get_l2_tps(db_pool: &DbPool) -> Result<Decimal> {
    let stmt = format!(
        "select extract(epoch from (now() - min(created_at))), sum(tx_num)
        from {} where created_at >= now() - interval '1' hour",
        table_name::L2_BLOCK
    );
    match query_as::<_, (Option<f64>, Option<Decimal>)>(&stmt)
        .fetch_one(db_pool)
        .await
    {
        Ok((secs, tx_num)) => {
            log::debug!("l2_tps: secs = {secs:?}, tx_num = {tx_num:?}");
            let secs = secs
                .and_then(Decimal::from_f64_retain)
                .unwrap_or(Decimal::ZERO);
            Ok(tx_num
                .and_then(|tn| tn.checked_div(secs))
                .unwrap_or(Decimal::ZERO))
        }
        Err(RowNotFound) => Ok(Decimal::ZERO),
        Err(error) => Err(error),
    }
}

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
