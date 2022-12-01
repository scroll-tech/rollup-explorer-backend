use crate::cache::*;
use crate::db::models::BlockTrace;
use crate::open_api::objects::Block;
use poem_openapi::Object;

#[derive(Clone, Debug, Object)]
pub struct BlocksResponse {
    batch_index: i64,
    blocks: Vec<Block>,
}

impl BlocksResponse {
    pub fn new(batch_index: i64, block_traces: Vec<BlockTrace>) -> Self {
        let blocks = block_traces.into_iter().map(Into::into).collect();

        Self {
            batch_index,
            blocks,
        }
    }

    pub async fn from_cache(cache: &Cache, cache_key: &str) -> Option<Self> {
        from_cache(cache, cache_key).await
    }
}
