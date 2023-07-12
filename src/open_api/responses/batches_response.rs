use crate::{cache::*, db::models, open_api::objects::Batch};
use poem_openapi::Object;

#[derive(Clone, Debug, Object)]
pub struct BatchesResponse {
    total: i64,
    batches: Vec<Batch>,
}

impl BatchesResponse {
    pub fn new(total: i64, batches: Vec<models::Batch>) -> Self {
        let batches = batches.into_iter().map(Into::into).collect();

        Self { total, batches }
    }

    pub async fn from_cache(cache: &Cache, cache_key: &str) -> Option<Self> {
        from_cache(cache, cache_key).await
    }
}
