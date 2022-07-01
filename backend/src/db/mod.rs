pub mod migrator;
pub mod models;
pub mod queries;

pub use queries::*;

pub type DbPool = sqlx::postgres::PgPool;

pub mod table_name {
    pub const L2_BLOCK: &str = "l2_blocks";
}
