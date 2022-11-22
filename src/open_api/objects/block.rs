use crate::db::models::BlockTrace;
use poem_openapi::Object;
use rust_decimal::Decimal;

#[derive(Clone, Debug, Object)]
pub struct Block {
    number: i64,
    tx_num: i32,
    hash: String,
    block_timestamp: Decimal,
}

impl From<BlockTrace> for Block {
    fn from(block_trace: BlockTrace) -> Self {
        Self {
            number: block_trace.number,
            tx_num: block_trace.tx_num,
            hash: block_trace.hash,
            block_timestamp: block_trace.block_timestamp,
        }
    }
}
