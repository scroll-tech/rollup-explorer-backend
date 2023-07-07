use crate::db::{models, RollupStatusType};
use poem_openapi::Object;
use rust_decimal::Decimal;
use std::fmt;

#[derive(Clone, Debug)]
pub enum RollupStatus {
    Precommitted,
    Committed,
    Finalized,
    Skipped,
    Unknown,
}

impl fmt::Display for RollupStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Precommitted => "precommitted",
            Self::Committed => "committed",
            Self::Finalized => "finalized",
            Self::Skipped => "skipped",
            Self::Unknown => "unknown",
        };
        write!(f, "{s}")
    }
}

impl From<RollupStatusType> for RollupStatus {
    fn from(t: RollupStatusType) -> Self {
        match t {
            1 | 2 => Self::Precommitted,
            3 | 4 => Self::Committed,
            5 => Self::Finalized,
            6 => Self::Skipped,
            _ => Self::Unknown,
        }
    }
}

#[derive(Clone, Debug, Object)]
pub struct Batch {
    hash: String,
    index: i64,
    start_chunk_index: i64,
    end_chunk_index: i64,
    start_block_number: i64,
    end_block_number: i64,
    total_tx_num: i64,
    rollup_status: String,
    commit_tx_hash: Option<String>,
    finalize_tx_hash: Option<String>,
    created_at: Decimal,
    committed_at: Option<Decimal>,
    finalized_at: Option<Decimal>,
}

impl From<models::Batch> for Batch {
    fn from(batch: models::Batch) -> Self {
        Self {
            hash: batch.hash,
            index: batch.index,
            start_chunk_index: batch.start_chunk_index,
            end_chunk_index: batch.end_chunk_index,
            start_block_number: batch.start_block_number.unwrap_or_default(),
            end_block_number: batch.end_block_number.unwrap_or_default(),
            total_tx_num: batch.total_tx_num.unwrap_or_default(),
            rollup_status: RollupStatus::from(batch.rollup_status).to_string(),
            commit_tx_hash: batch.commit_tx_hash,
            finalize_tx_hash: batch.finalize_tx_hash,
            created_at: batch.created_at.timestamp().into(),
            committed_at: batch.committed_at.map(|t| t.timestamp().into()),
            finalized_at: batch.finalized_at.map(|t| t.timestamp().into()),
        }
    }
}
