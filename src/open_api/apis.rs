use crate::{
    consts::*,
    db::*,
    open_api::{responses::*, State},
};
use poem::{error::InternalServerError, web::Data, Result};
use poem_openapi::{param::Query, payload::Json, OpenApi};
use std::sync::Arc;

// Macro used to log error with right line number.
macro_rules! api_err {
    ($err:expr) => {{
        log::error!("{:?}", $err);
        InternalServerError($err)
    }};
}

pub(crate) struct Apis;

#[OpenApi]
impl Apis {
    #[oai(path = "/batch", method = "get")]
    async fn batch(&self, state: Data<&State>, index: Query<i64>) -> Result<Json<BatchResponse>> {
        let index = index.0;

        // Return directly if cached.
        let cache_key = format!("batch-{index}");
        if let Some(response) = BatchResponse::from_cache(state.cache.as_ref(), &cache_key).await {
            log::debug!("OpenAPI - Get batch from Cache: {response:?}");
            return Ok(Json(response));
        };

        let batch = batch_query::fetch_one(&state.db_pool, index)
            .await
            .map_err(|e| api_err!(e))?;
        let response = BatchResponse::new(batch);

        // Save to cache.
        if let Err(error) = state
            .cache
            .set(
                &cache_key,
                Arc::new(response.clone()),
                DEFAULT_CACHE_EXPIRED_SECS,
            )
            .await
        {
            log::error!("OpenAPI - Failed to save cache of {cache_key}: {error}");
        }

        Ok(Json(response))
    }

    #[oai(path = "/batches", method = "get")]
    async fn batches(
        &self,
        state: Data<&State>,
        page: Query<Option<u64>>,
        per_page: Query<Option<u64>>,
    ) -> Result<Json<BatchesResponse>> {
        let limit = per_page.0.map_or_else(
            || DEFAULT_PER_PAGE,
            |val| {
                if val > 0 {
                    if val > state.max_per_page {
                        state.max_per_page
                    } else {
                        val
                    }
                } else {
                    DEFAULT_PER_PAGE
                }
            },
        );
        let offset = page
            .0
            .map_or_else(|| 0, |val| if val > 0 { (val - 1) * limit } else { 0 });

        // Return directly if cached.
        let cache_key = format!("batches-of-offset-{offset}-limit-{limit}");
        if let Some(response) = BatchesResponse::from_cache(state.cache.as_ref(), &cache_key).await
        {
            log::debug!("OpenAPI - Get batches from Cache: {response:?}");
            return Ok(Json(response));
        };

        let total = batch_query::get_total(&state.db_pool)
            .await
            .map_err(|e| api_err!(e))?;
        let batches = batch_query::fetch_all(&state.db_pool, offset, limit)
            .await
            .map_err(|e| api_err!(e))?;
        let response = BatchesResponse::new(total, batches);

        // Save to cache.
        if let Err(error) = state
            .cache
            .set(
                &cache_key,
                Arc::new(response.clone()),
                DEFAULT_CACHE_EXPIRED_SECS,
            )
            .await
        {
            log::error!("OpenAPI - Failed to save cache of {cache_key}: {error}");
        }

        Ok(Json(response))
    }

    #[oai(path = "/blocks", method = "get")]
    async fn blocks(
        &self,
        state: Data<&State>,
        chunk_index: Query<i64>,
    ) -> Result<Json<BlocksResponse>> {
        let chunk_index = chunk_index.0;

        // Return directly if cached.
        let cache_key = format!("blocks-of-chunk-{chunk_index}");
        if let Some(response) = BlocksResponse::from_cache(state.cache.as_ref(), &cache_key).await {
            log::debug!("OpenAPI - Get blocks from Cache: {response:?}");
            return Ok(Json(response));
        };

        let chunk_hash = chunk_query::get_hash_by_index(&state.db_pool, chunk_index)
            .await
            .map_err(|e| api_err!(e))?;
        let (chunk_index, l2_blocks) = if let Some(hash) = chunk_hash {
            (
                chunk_index,
                l2_block_query::fetch_all(&state.db_pool, &hash)
                    .await
                    .map_err(|e| api_err!(e))?,
            )
        } else {
            (INVALID_IDNEX, vec![])
        };
        let response = BlocksResponse::new(chunk_index, l2_blocks);

        // Save to cache.
        if let Err(error) = state
            .cache
            .set(
                &cache_key,
                Arc::new(response.clone()),
                DEFAULT_CACHE_EXPIRED_SECS,
            )
            .await
        {
            log::error!("OpenAPI - Failed to save cache of {cache_key}: {error}");
        }

        Ok(Json(response))
    }

    #[oai(path = "/chunks", method = "get")]
    async fn chunks(
        &self,
        state: Data<&State>,
        batch_index: Query<i64>,
    ) -> Result<Json<ChunksResponse>> {
        let batch_index = batch_index.0;

        // Return directly if cached.
        let cache_key = format!("chunks-of-batch-{batch_index}");
        if let Some(response) = ChunksResponse::from_cache(state.cache.as_ref(), &cache_key).await {
            log::debug!("OpenAPI - Get chunks from Cache: {response:?}");
            return Ok(Json(response));
        };

        let batch_hash = batch_query::get_hash_by_index(&state.db_pool, batch_index)
            .await
            .map_err(|e| api_err!(e))?;
        let (batch_index, chunks) = if let Some(hash) = batch_hash {
            (
                batch_index,
                chunk_query::fetch_all(&state.db_pool, &hash)
                    .await
                    .map_err(|e| api_err!(e))?,
            )
        } else {
            (INVALID_IDNEX, vec![])
        };
        let response = ChunksResponse::new(batch_index, chunks);

        // Save to cache.
        if let Err(error) = state
            .cache
            .set(
                &cache_key,
                Arc::new(response.clone()),
                DEFAULT_CACHE_EXPIRED_SECS,
            )
            .await
        {
            log::error!("OpenAPI - Failed to save cache of {cache_key}: {error}");
        }

        Ok(Json(response))
    }

    #[oai(path = "/last_batch_indexes", method = "get")]
    async fn last_batch_indexes(
        &self,
        state: Data<&State>,
    ) -> Result<Json<LastBatchIndexesResponse>> {
        // Return directly if cached.
        if let Some(response) =
            LastBatchIndexesResponse::from_cache(&state.cache, "last_batch_indexes").await
        {
            log::debug!("OpenAPI - Get last batch indexes from Cache: {response:?}");
            return Ok(Json(response));
        };

        let status_indexes = batch_query::get_max_status_indexes(&state.db_pool)
            .await
            .map_err(|e| api_err!(e))?;
        let response = LastBatchIndexesResponse::new(status_indexes);

        // Save to cache.
        if let Err(error) = state
            .cache
            .set(
                "last_batch_indexes",
                Arc::new(response.clone()),
                DEFAULT_CACHE_EXPIRED_SECS,
            )
            .await
        {
            log::error!("OpenAPI - Failed to save cache for last batch indexes: {error}");
        }

        Ok(Json(response))
    }

    // Parameter `keyword` should be a block number or block hash in `l2block`
    // table.
    #[oai(path = "/search", method = "get")]
    async fn search(
        &self,
        state: Data<&State>,
        keyword: Query<String>,
    ) -> Result<Json<SearchResponse>> {
        let keyword = keyword.0;

        // Return directly if cached.
        let cache_key = format!("search-{keyword}");
        if let Some(response) = SearchResponse::from_cache(state.cache.as_ref(), &cache_key).await {
            log::debug!("OpenAPI - Get blocks from Cache: {response:?}");
            return Ok(Json(response));
        };

        // Consider `keyword` as block number if it is an integer, otherwise
        // consider as block hash (starts as `0x`).
        let batch_hash = match keyword.parse::<i64>() {
            Ok(block_num) => l2_block_query::get_batch_hash_by_number(&state.db_pool, block_num)
                .await
                .map_err(|e| api_err!(e))?,
            Err(_) => l2_block_query::get_batch_hash_by_block_hash(&state.db_pool, &keyword)
                .await
                .map_err(|e| api_err!(e))?,
        };

        let batch_index = if let Some(hash) = batch_hash {
            batch_query::get_index_by_hash(&state.db_pool, &hash)
                .await
                .map_err(|e| api_err!(e))?
                .unwrap_or(INVALID_IDNEX)
        } else {
            INVALID_IDNEX
        };
        let response = SearchResponse::new(batch_index);

        // Save to cache.
        if let Err(error) = state
            .cache
            .set(
                &cache_key,
                Arc::new(response.clone()),
                DEFAULT_CACHE_EXPIRED_SECS,
            )
            .await
        {
            log::error!("OpenAPI - Failed to save cache of {cache_key}: {error}");
        }

        Ok(Json(response))
    }
}
