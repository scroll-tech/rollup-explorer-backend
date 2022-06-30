use crate::db::models::BlockStatus;
use crate::db::{table_name, DbPool};
use rust_decimal::Decimal;
use sqlx::Error::RowNotFound;
use sqlx::{query_as, Result};

pub async fn get_l1_tps(db_pool: &DbPool) -> Result<Decimal> {
    let stmt = format!(
        "select extract(epoch from (now() - min(updated_at))), sum(tx_num)
        from {} where status = $1 and updated_at >= now() - interval '1' hour",
        table_name::L2_BLOCK,
    );
    let res = query_as::<_, (Option<f64>, Option<Decimal>)>(&stmt)
        .bind(BlockStatus::Verified)
        .fetch_one(db_pool)
        .await;
    calculate_tps_by_sql_result(res)
}

pub async fn get_l2_tps(db_pool: &DbPool) -> Result<Decimal> {
    let stmt = format!(
        "select extract(epoch from (now() - min(created_at))), sum(tx_num)
        from {} where created_at >= now() - interval '1' hour",
        table_name::L2_BLOCK
    );
    let res = query_as::<_, (Option<f64>, Option<Decimal>)>(&stmt)
        .fetch_one(db_pool)
        .await;
    calculate_tps_by_sql_result(res)
}

fn calculate_tps_by_sql_result(
    sql_result: Result<(Option<f64>, Option<Decimal>)>,
) -> Result<Decimal> {
    match sql_result {
        Ok((secs, tx_num)) => {
            log::debug!("TPS: secs = {secs:?}, tx_num = {tx_num:?}");
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
