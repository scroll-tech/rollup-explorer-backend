use crate::{cache::*, db::models, open_api::objects::Chunk};
use poem_openapi::Object;

#[derive(Clone, Debug, Object)]
pub struct ChunksResponse {
    batch_index: i64,
    chunks: Vec<Chunk>,
}

impl ChunksResponse {
    pub fn new(batch_index: i64, chunks: Vec<models::Chunk>) -> Self {
        let chunks = chunks.into_iter().map(Into::into).collect();

        Self {
            batch_index,
            chunks,
        }
    }

    pub async fn from_cache(cache: &Cache, cache_key: &str) -> Option<Self> {
        from_cache(cache, cache_key).await
    }
}
