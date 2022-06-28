use crate::db::models;
use poem_openapi::Object;
use rust_decimal::Decimal;

#[derive(Object)]
pub struct L2Block {
    block_height: i64,
    tx_num: i64,
    status: String,
    header_hash: String,
    l1_tx_hash: String,
    timestamp: Decimal,
}

impl From<models::L2Block> for L2Block {
    fn from(model: models::L2Block) -> Self {
        Self {
            block_height: model.id,
            tx_num: model.tx_num,
            status: model.status.to_string(),
            header_hash: model.header_hash,
            l1_tx_hash: model.l1_tx_hash.unwrap_or_else(|| "".to_owned()),
            timestamp: model.timestamp,
        }
    }
}
