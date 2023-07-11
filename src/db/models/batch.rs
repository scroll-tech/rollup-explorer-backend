use crate::db::RollupStatusType;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(sqlx::FromRow, Clone, Debug, Serialize)]
pub struct Batch {
    pub hash: String,
    pub index: i64,
    pub start_chunk_index: i64,
    pub end_chunk_index: i64,
    #[sqlx(default)]
    pub start_block_number: Option<i64>,
    #[sqlx(default)]
    pub end_block_number: Option<i64>,
    #[sqlx(default)]
    pub total_tx_num: Option<i64>,
    pub rollup_status: RollupStatusType,
    pub commit_tx_hash: Option<String>,
    pub finalize_tx_hash: Option<String>,
    pub created_at: NaiveDateTime,
    pub committed_at: Option<NaiveDateTime>,
    pub finalized_at: Option<NaiveDateTime>,
}
