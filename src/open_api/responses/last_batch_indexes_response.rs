use crate::{cache::*, db::RollupStatusType, open_api::objects::RollupStatus};
use poem_openapi::Object;
use std::collections::HashMap;

#[derive(Clone, Debug, Object)]
pub struct LastBatchIndexesResponse {
    all_index: i64,
    committed_index: i64,
    finalized_index: i64,
}

impl LastBatchIndexesResponse {
    pub fn new(status_indexes: HashMap<RollupStatusType, i64>) -> Self {
        let mut all_index = 0;
        let mut committed_index = 0;
        let mut finalized_index = 0;

        for (status, index) in status_indexes.into_iter() {
            all_index = all_index.max(index);
            match status.into() {
                RollupStatus::Committed => committed_index = committed_index.max(index),
                RollupStatus::Finalized => finalized_index = finalized_index.max(index),
                _ => (),
            }
        }

        // Set `committed` index as maximum index of both committed and
        // finalized batches.
        committed_index = committed_index.max(finalized_index);

        Self {
            all_index,
            committed_index,
            finalized_index,
        }
    }

    pub async fn from_cache(cache: &Cache, cache_key: &str) -> Option<Self> {
        from_cache(cache, cache_key).await
    }
}
