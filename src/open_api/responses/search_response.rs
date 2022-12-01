use crate::cache::*;
use poem_openapi::Object;

#[derive(Clone, Debug, Object)]
pub struct SearchResponse {
    batch_index: i64,
}

impl SearchResponse {
    pub fn new(batch_index: i64) -> Self {
        Self { batch_index }
    }

    pub async fn from_cache(cache: &Cache, cache_key: &str) -> Option<Self> {
        from_cache(cache, cache_key).await
    }
}
