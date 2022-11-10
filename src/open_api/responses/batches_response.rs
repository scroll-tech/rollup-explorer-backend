use crate::cache::Cache;
use crate::db::models::BlockBatch;
use crate::open_api::objects::Batch;
use poem_openapi::Object;

#[derive(Clone, Debug, Object)]
pub struct BatchesResponse {
    total: i32,
    batches: Vec<Batch>,
}

impl BatchesResponse {
    pub fn new(total: i32, block_batches: Vec<BlockBatch>) -> Self {
        let batches = block_batches.into_iter().map(Into::into).collect();

        Self { total, batches }
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
