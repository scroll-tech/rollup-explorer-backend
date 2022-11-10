use crate::db::models::BlockResult;
use poem_openapi::Object;
use rust_decimal::Decimal;

#[derive(Clone, Debug, Object)]
pub struct Block {
    number: i64,
    tx_num: i32,
    hash: String,
    block_timestamp: Decimal,
}

impl From<BlockResult> for Block {
    fn from(block_result: BlockResult) -> Self {
        Self {
            number: block_result.number,
            tx_num: block_result.tx_num,
            hash: block_result.hash,
            block_timestamp: block_result.block_timestamp,
        }
    }
}
