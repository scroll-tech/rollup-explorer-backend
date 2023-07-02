use crate::db::models;
use poem_openapi::Object;
use rust_decimal::Decimal;

#[derive(Clone, Debug, Object)]
pub struct Block {
    number: i64,
    tx_num: i32,
    hash: String,
    block_timestamp: Decimal,
}

impl From<models::L2Block> for Block {
    fn from(l2_block: models::L2Block) -> Self {
        Self {
            number: l2_block.number,
            tx_num: l2_block.tx_num,
            hash: l2_block.hash,
            block_timestamp: l2_block.block_timestamp,
        }
    }
}
