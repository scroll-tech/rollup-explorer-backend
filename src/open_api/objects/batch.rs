use crate::db::models::BlockBatch;
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

impl From<i64> for RollupStatus {
    fn from(i: i64) -> Self {
        match i {
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
    id: String,
    index: i64,
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

impl From<BlockBatch> for Batch {
    fn from(block_batch: BlockBatch) -> Self {
        Self {
            id: block_batch.id,
            index: block_batch.index,
            start_block_number: block_batch.start_block_number,
            end_block_number: block_batch.end_block_number,
            total_tx_num: block_batch.total_tx_num,
            rollup_status: RollupStatus::from(block_batch.rollup_status).to_string(),
            commit_tx_hash: block_batch.commit_tx_hash,
            finalize_tx_hash: block_batch.finalize_tx_hash,
            created_at: block_batch.created_at.timestamp().into(),
            committed_at: block_batch.committed_at.map(|t| t.timestamp().into()),
            finalized_at: block_batch.finalized_at.map(|t| t.timestamp().into()),
        }
    }
}
