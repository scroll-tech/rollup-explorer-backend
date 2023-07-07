use rust_decimal::Decimal;
use serde::Serialize;

#[derive(sqlx::FromRow, Clone, Debug, Serialize)]
pub struct Block {
    pub number: i64,
    pub tx_num: i32,
    pub hash: String,
    pub chunk_hash: Option<String>,
    pub block_timestamp: Decimal,
}
