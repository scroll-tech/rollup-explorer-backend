use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(sqlx::FromRow, Clone, Debug, Serialize)]
pub struct Chunk {
    pub index: i64,
    pub start_block_number: i64,
    pub end_block_number: i64,
    pub total_tx_num: i64,
    pub hash: String,
    pub batch_hash: Option<String>,
    pub created_at: NaiveDateTime,
}
