use crate::cache::*;
use crate::db::models::BlockBatch;
use crate::open_api::objects::Batch;
use poem_openapi::Object;

#[derive(Clone, Debug, Object)]
pub struct BatchesResponse {
    total: i64,
    batches: Vec<Batch>,
}

impl BatchesResponse {
    pub fn new(total: i64, block_batches: Vec<BlockBatch>) -> Self {
        let batches = block_batches.into_iter().map(Into::into).collect();

        Self { total, batches }
    }

    pub async fn from_cache(cache: &Cache, cache_key: &str) -> Option<Self> {
        from_cache(cache, cache_key).await
    }
}
