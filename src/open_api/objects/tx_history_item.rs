use crate::db::models::LayerMsg;
use poem_openapi::Object;

#[derive(Clone, Debug, Object)]
pub struct TxHistoryItem {
    height: i64,
    status: String,
    layer1_hash: String,
    layer2_hash: String,
    event: String,
}

impl From<LayerMsg> for TxHistoryItem {
    fn from(msg: LayerMsg) -> Self {
        let status = match msg.status {
            1 => "pending",
            2 => "submitted",
            3 => "confirmed",
            _ => "undefined",
        }
        .to_string();

        Self {
            height: msg.height,
            status,
            layer1_hash: msg.layer1_hash.unwrap_or_default(),
            layer2_hash: msg.layer2_hash.unwrap_or_default(),
            event: msg.event,
        }
    }
}
