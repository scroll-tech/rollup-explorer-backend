use crate::db::RollupStatusType;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(sqlx::FromRow, Clone, Debug, Serialize)]
pub struct Batch {
    pub hash: String,
    pub index: i64,
    pub start_chunk_index: i64,
    pub end_chunk_index: i64,
    pub start_block_number: Option<i64>,
    pub end_block_number: Option<i64>,
    pub total_tx_num: Option<Decimal>,
    pub rollup_status: RollupStatusType,
    pub commit_tx_hash: Option<String>,
    pub finalize_tx_hash: Option<String>,
    pub created_at: NaiveDateTime,
    pub committed_at: Option<NaiveDateTime>,
    pub finalized_at: Option<NaiveDateTime>,
}
