use rust_decimal::Decimal;
use serde::Serialize;
use std::fmt;

#[derive(sqlx::Type, Clone, Debug, Serialize)]
#[repr(i32)]
pub enum BlockStatus {
    Unassigned = 0,
    Assigned = 1,
    Proved = 2,
    Verified = 3,
    Submitted = 4,
}

impl fmt::Display for BlockStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(sqlx::FromRow, Clone, Debug, Serialize)]
pub struct BlockResult {
    pub number: i32,
    pub tx_num: i64,
    pub hash: String,
    pub status: BlockStatus,
    pub block_timestamp: Decimal,
}
