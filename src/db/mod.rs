mod queries;

pub mod models;

pub use queries::*;

pub type DbPool = sqlx::postgres::PgPool;
pub type RollupStatusType = i32;

pub mod table_name {
    pub const BLOCK_BATCH: &str = "block_Batch";
    pub const BLOCK_RESULT: &str = "block_result";
}
