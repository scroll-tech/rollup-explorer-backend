use crate::cache::Cache;
use crate::db::models::LayerMsg;
use crate::open_api::objects::TxHistoryItem;
use poem_openapi::Object;

#[derive(Clone, Debug, Object)]
pub struct TxHistoryResponse {
    items: Vec<TxHistoryItem>,
}

impl TxHistoryResponse {
    pub fn new(msgs: Vec<LayerMsg>) -> Self {
        let items = msgs.into_iter().map(Into::into).collect();
        Self { items }
    }

    pub async fn from_cache(cache: &Cache, cache_key: &str) -> Option<Self> {
        cache
            .get(cache_key)
            .await
            .ok()
            .flatten()
            .and_then(|any| any.downcast_ref::<Self>().cloned())
    }
}
