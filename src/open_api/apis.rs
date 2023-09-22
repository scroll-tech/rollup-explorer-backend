use crate::{
    consts::*,
    db::*,
    open_api::{responses::*, State, CACHE_HITS, INCOMING_REQUESTS, RESPONSE_TIME_COLLECTOR},
};
use poem::{error::InternalServerError, web::Data, Result};
use poem_openapi::{param::Query, payload::Json, OpenApi};
use std::{sync::Arc, time::Instant};

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
        let response_time = Instant::now();
        INCOMING_REQUESTS.inc();

        let index = index.0;

        // Return directly if cached.
        let cache_key = format!("batch-{index}");
        if let Some(response) = BatchResponse::from_cache(state.cache.as_ref(), &cache_key).await {
            log::debug!("OpenAPI - Get batch from Cache: {response:?}");
            CACHE_HITS.with_label_values(&[cache_key.as_str()]).inc();

            RESPONSE_TIME_COLLECTOR
                .with_label_values(&["batch"])
                .observe(response_time.elapsed().as_secs_f64());

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

        RESPONSE_TIME_COLLECTOR
            .with_label_values(&["batch"])
            .observe(response_time.elapsed().as_secs_f64());

        Ok(Json(response))
    }

    #[oai(path = "/batches", method = "get")]
    async fn batches(
        &self,
        state: Data<&State>,
        page: Query<Option<u64>>,
        per_page: Query<Option<u64>>,
    ) -> Result<Json<BatchesResponse>> {
        let response_time = Instant::now();
        INCOMING_REQUESTS.inc();

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
            CACHE_HITS.with_label_values(&[cache_key.as_str()]).inc();

            RESPONSE_TIME_COLLECTOR
                .with_label_values(&["batches"])
                .observe(response_time.elapsed().as_secs_f64());

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

        RESPONSE_TIME_COLLECTOR
            .with_label_values(&["batches"])
            .observe(response_time.elapsed().as_secs_f64());

        Ok(Json(response))
    }

    #[oai(path = "/batch_blocks", method = "get")]
    async fn batch_blocks(
        &self,
        state: Data<&State>,
        batch_index: Query<i64>,
    ) -> Result<Json<BlocksResponse>> {
        let response_time = Instant::now();
        INCOMING_REQUESTS.inc();

        let batch_index = batch_index.0;

        // Return directly if cached.
        let cache_key = format!("blocks-of-batch-{batch_index}");
        if let Some(response) = BlocksResponse::from_cache(state.cache.as_ref(), &cache_key).await {
            log::debug!("OpenAPI - Get blocks from Cache: {response:?}");
            CACHE_HITS.with_label_values(&[cache_key.as_str()]).inc();

            RESPONSE_TIME_COLLECTOR
                .with_label_values(&["batch_blocks"])
                .observe(response_time.elapsed().as_secs_f64());

            return Ok(Json(response));
        };

        let response = query_blocks_by_batch_index(&state.db_pool, batch_index).await?;

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

        RESPONSE_TIME_COLLECTOR
            .with_label_values(&["batch_blocks"])
            .observe(response_time.elapsed().as_secs_f64());

        Ok(Json(response))
    }

    #[oai(path = "/chunks", method = "get")]
    async fn chunks(
        &self,
        state: Data<&State>,
        batch_index: Query<i64>,
    ) -> Result<Json<ChunksResponse>> {
        let response_time = Instant::now();
        INCOMING_REQUESTS.inc();

        let batch_index = batch_index.0;

        // Return directly if cached.
        let cache_key = format!("chunks-of-batch-{batch_index}");
        if let Some(response) = ChunksResponse::from_cache(state.cache.as_ref(), &cache_key).await {
            log::debug!("OpenAPI - Get chunks from Cache: {response:?}");
            CACHE_HITS.with_label_values(&[cache_key.as_str()]).inc();

            RESPONSE_TIME_COLLECTOR
                .with_label_values(&["chunks"])
                .observe(response_time.elapsed().as_secs_f64());

            return Ok(Json(response));
        };

        let batch_hash = batch_query::get_hash_by_index(&state.db_pool, batch_index)
            .await
            .map_err(|e| api_err!(e))?;
        let (batch_index, chunks) = if let Some(hash) = batch_hash {
            (
                batch_index,
                chunk_query::fetch_by_batch_hash(&state.db_pool, &hash)
                    .await
                    .map_err(|e| api_err!(e))?,
            )
        } else {
            (INVALID_INDEX, vec![])
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

        RESPONSE_TIME_COLLECTOR
            .with_label_values(&["chunks"])
            .observe(response_time.elapsed().as_secs_f64());

        Ok(Json(response))
    }

    #[oai(path = "/chunk_blocks", method = "get")]
    async fn chunk_blocks(
        &self,
        state: Data<&State>,
        chunk_index: Query<i64>,
    ) -> Result<Json<BlocksResponse>> {
        let response_time = Instant::now();
        INCOMING_REQUESTS.inc();

        let chunk_index = chunk_index.0;

        // Return directly if cached.
        let cache_key = format!("blocks-of-chunk-{chunk_index}");
        if let Some(response) = BlocksResponse::from_cache(state.cache.as_ref(), &cache_key).await {
            log::debug!("OpenAPI - Get blocks from Cache: {response:?}");
            CACHE_HITS.with_label_values(&[cache_key.as_str()]).inc();

            RESPONSE_TIME_COLLECTOR
                .with_label_values(&["chunk_blocks"])
                .observe(response_time.elapsed().as_secs_f64());

            return Ok(Json(response));
        };

        let response = query_blocks_by_chunk_index(&state.db_pool, chunk_index).await?;

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

        RESPONSE_TIME_COLLECTOR
            .with_label_values(&["chunk_blocks"])
            .observe(response_time.elapsed().as_secs_f64());

        Ok(Json(response))
    }

    #[oai(path = "/last_batch_indexes", method = "get")]
    async fn last_batch_indexes(
        &self,
        state: Data<&State>,
    ) -> Result<Json<LastBatchIndexesResponse>> {
        let response_time = Instant::now();
        INCOMING_REQUESTS.inc();

        // Return directly if cached.
        if let Some(response) =
            LastBatchIndexesResponse::from_cache(&state.cache, "last_batch_indexes").await
        {
            log::debug!("OpenAPI - Get last batch indexes from Cache: {response:?}");
            CACHE_HITS.with_label_values(&["last_batch_indexes"]).inc();

            RESPONSE_TIME_COLLECTOR
                .with_label_values(&["last_batch_indexes"])
                .observe(response_time.elapsed().as_secs_f64());

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

        RESPONSE_TIME_COLLECTOR
            .with_label_values(&["last_batch_indexes"])
            .observe(response_time.elapsed().as_secs_f64());

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
        let response_time = Instant::now();
        INCOMING_REQUESTS.inc();

        let keyword = keyword.0;

        // Return directly if cached.
        let cache_key = format!("search-{keyword}");
        if let Some(response) = SearchResponse::from_cache(state.cache.as_ref(), &cache_key).await {
            log::debug!("OpenAPI - Get blocks from Cache: {response:?}");
            CACHE_HITS.with_label_values(&[cache_key.as_str()]).inc();

            RESPONSE_TIME_COLLECTOR
                .with_label_values(&["search"])
                .observe(response_time.elapsed().as_secs_f64());

            return Ok(Json(response));
        };

        // Consider `keyword` as block number if it is an integer, otherwise
        // consider as block hash (starts as `0x`).
        let batch_hash = match keyword.parse::<i64>() {
            Ok(block_num) => block_query::get_batch_hash_by_number(&state.db_pool, block_num)
                .await
                .map_err(|e| api_err!(e))?,
            Err(_) => block_query::get_batch_hash_by_block_hash(&state.db_pool, &keyword)
                .await
                .map_err(|e| api_err!(e))?,
        };

        let batch_index = if let Some(hash) = batch_hash {
            batch_query::get_index_by_hash(&state.db_pool, &hash)
                .await
                .map_err(|e| api_err!(e))?
                .unwrap_or(INVALID_INDEX)
        } else {
            INVALID_INDEX
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

        RESPONSE_TIME_COLLECTOR
            .with_label_values(&["search"])
            .observe(response_time.elapsed().as_secs_f64());

        Ok(Json(response))
    }
}

async fn query_blocks_by_batch_index(db_pool: &DbPool, batch_index: i64) -> Result<BlocksResponse> {
    let batch_hash = batch_query::get_hash_by_index(db_pool, batch_index)
        .await
        .map_err(|e| api_err!(e))?;
    if batch_hash.is_none() {
        return Ok(BlocksResponse::from_batch_blocks(INVALID_INDEX, vec![]));
    }
    let batch_hash = batch_hash.unwrap();

    let (start_block_num, end_block_num) =
        chunk_query::get_block_num_range_by_batch_hash(db_pool, &batch_hash)
            .await
            .map_err(|e| api_err!(e))?;

    let blocks = block_query::fetch_by_num_range(db_pool, start_block_num, end_block_num)
        .await
        .map_err(|e| api_err!(e))?;

    Ok(BlocksResponse::from_batch_blocks(batch_index, blocks))
}

async fn query_blocks_by_chunk_index(db_pool: &DbPool, chunk_index: i64) -> Result<BlocksResponse> {
    let chunk_hash = chunk_query::get_hash_by_index(db_pool, chunk_index)
        .await
        .map_err(|e| api_err!(e))?;
    if chunk_hash.is_none() {
        return Ok(BlocksResponse::from_chunk_blocks(INVALID_INDEX, vec![]));
    }
    let chunk_hash = chunk_hash.unwrap();

    let blocks = block_query::fetch_by_chunk_hash(db_pool, &chunk_hash)
        .await
        .map_err(|e| api_err!(e))?;

    Ok(BlocksResponse::from_chunk_blocks(chunk_index, blocks))
}
