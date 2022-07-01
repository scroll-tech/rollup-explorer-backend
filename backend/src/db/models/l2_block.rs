use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Serialize;
use std::fmt;

#[derive(sqlx::Type, Clone, Debug, Serialize)]
#[sqlx(type_name = "block_status", rename_all = "snake_case")]
pub enum BlockStatus {
    Uncommitted,
    Committed,
    Verified,
}

impl fmt::Display for BlockStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(sqlx::FromRow, Clone, Debug, Serialize)]
pub struct L2Block {
    pub id: i64,
    pub status: BlockStatus,
    pub header_hash: String,
    pub l1_tx_hash: Option<String>,
    pub tx_num: i64,
    pub timestamp: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
