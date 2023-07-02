use crate::db::models;
use poem_openapi::Object;
use rust_decimal::Decimal;
use std::fmt;

#[derive(Clone, Debug, Object)]
pub struct Chunk {
    hash: String,
    index: i64,
    start_block_number: i64,
    end_block_number: i64,
    total_l2_tx_num: i64,
    created_at: Decimal,
}

impl From<models::Chunk> for Chunk {
    fn from(chunk: models::Chunk) -> Self {
        Self {
            hash: chunk.hash,
            index: chunk.index,
            start_block_number: chunk.start_block_number,
            end_block_number: chunk.end_block_number,
            total_l2_tx_num: chunk.total_l2_tx_num,
            created_at: chunk.created_at.timestamp().into(),
        }
    }
}
