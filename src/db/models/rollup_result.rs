use chrono::NaiveDateTime;
use serde::Serialize;
use std::fmt;

#[derive(sqlx::FromRow, Clone, Debug, Serialize)]
pub struct RollupResult {
    pub number: i32,
    pub status: RollupStatus,
    pub rollup_tx_hash: Option<String>,
    pub finalize_tx_hash: Option<String>,
    pub created_time: NaiveDateTime,
    pub updated_time: NaiveDateTime,
}

#[derive(sqlx::Type, Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[repr(i32)]
pub enum RollupStatus {
    Undefined = 0,
    Pending,
    Committing,
    Committed,
    Finalizing,
    Finalized,
    FinalizationSkipped,
}

impl fmt::Display for RollupStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&RollupStatus> for i32 {
    fn from(status: &RollupStatus) -> Self {
        *status as Self
    }
}

impl RollupStatus {
    pub fn map_from_str(s: &str) -> Vec<Self> {
        match s {
            "unknown" => vec![RollupStatus::Undefined],
            "precommitted" => vec![RollupStatus::Pending, RollupStatus::Committing],
            "committed" => vec![RollupStatus::Committed, RollupStatus::Finalizing],
            "finalized" => vec![RollupStatus::Finalized],
            "skipped" => vec![RollupStatus::FinalizationSkipped],
            _ => unreachable!(),
        }
    }

    pub fn map_to_str(&self) -> &str {
        match &self {
            RollupStatus::Undefined => "unknown",
            RollupStatus::Pending | RollupStatus::Committing => "precommitted",
            RollupStatus::Committed | RollupStatus::Finalizing => "committed",
            RollupStatus::Finalized => "finalized",
            RollupStatus::FinalizationSkipped => "skipped",
        }
    }
}
