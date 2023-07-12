use crate::{cache::*, db::models, open_api::objects::Batch};
use poem_openapi::Object;

#[derive(Clone, Debug, Object)]
pub struct BatchResponse {
    batch: Option<Batch>,
}

impl BatchResponse {
    pub fn new(batch: Option<models::Batch>) -> Self {
        let batch = batch.map(Into::into);

        Self { batch }
    }

    pub async fn from_cache(cache: &Cache, cache_key: &str) -> Option<Self> {
        from_cache(cache, cache_key).await
    }
}
