pub mod models;
pub mod queries;

pub use queries::*;

pub type DbPool = sqlx::postgres::PgPool;

pub mod table_name {
    pub const BLOCK_RESULT: &str = "block_result";
    pub const LAYER1_MESSAGE: &str = "layer1_message";
    pub const LAYER2_MESSAGE: &str = "layer2_message";
    pub const ROLLUP_RESULT: &str = "rollup_result";
}
