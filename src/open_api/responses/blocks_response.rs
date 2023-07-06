use crate::{cache::*, db::models, open_api::objects::Block};
use poem_openapi::Object;

#[derive(Clone, Debug, Object)]
pub struct BlocksResponse {
    batch_index: Option<i64>,
    chunk_index: Option<i64>,
    blocks: Vec<Block>,
}

impl BlocksResponse {
    pub fn from_batch_blocks(batch_index: i64, blocks: Vec<models::Block>) -> Self {
        let blocks = blocks.into_iter().map(Into::into).collect();

        Self {
            batch_index,
            chunk_index: None,
            blocks,
        }
    }
    pub fn from_chunk_blocks(chunk_index: i64, blocks: Vec<models::Block>) -> Self {
        let blocks = blocks.into_iter().map(Into::into).collect();

        Self {
            batch_index: None,
            chunk_index,
            blocks,
        }
    }

    pub async fn from_cache(cache: &Cache, cache_key: &str) -> Option<Self> {
        from_cache(cache, cache_key).await
    }
}
