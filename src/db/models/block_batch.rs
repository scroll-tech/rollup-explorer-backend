use crate::db::RollupStatusType;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(sqlx::FromRow, Clone, Debug, Serialize)]
pub struct BlockBatch {
    pub id: String,
    pub index: i64,
    pub start_block_number: i64,
    pub end_block_number: i64,
    pub total_tx_num: i64,
    pub rollup_status: RollupStatusType,
    pub commit_tx_hash: Option<String>,
    pub finalize_tx_hash: Option<String>,
    pub created_at: NaiveDateTime,
    pub committed_at: Option<NaiveDateTime>,
    pub finalized_at: Option<NaiveDateTime>,
}
