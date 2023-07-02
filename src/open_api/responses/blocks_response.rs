use crate::{cache::*, db::models, open_api::objects::Block};
use poem_openapi::Object;

#[derive(Clone, Debug, Object)]
pub struct BlocksResponse {
    chunk_index: i64,
    blocks: Vec<Block>,
}

impl BlocksResponse {
    pub fn new(chunk_index: i64, l2_blocks: Vec<models::L2Block>) -> Self {
        let blocks = l2_blocks.into_iter().map(Into::into).collect();

        Self {
            chunk_index,
            blocks,
        }
    }

    pub async fn from_cache(cache: &Cache, cache_key: &str) -> Option<Self> {
        from_cache(cache, cache_key).await
    }
}
