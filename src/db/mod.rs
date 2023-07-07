mod queries;

pub mod models;

pub use queries::*;

pub type DbPool = sqlx::postgres::PgPool;
pub type RollupStatusType = i32;

pub mod table_name {
    pub const BATCH: &str = "batch";
    pub const BLOCK: &str = "l2_block";
    pub const CHUNK: &str = "chunk";
}
