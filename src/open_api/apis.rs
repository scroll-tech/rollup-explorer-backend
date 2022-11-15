use crate::consts::*;
use crate::db::*;
use crate::open_api::responses::*;
use crate::open_api::State;
use poem::error::InternalServerError;
use poem::web::Data;
use poem::Result;
use poem_openapi::param::Query;
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
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
    async fn batch(
        &self,
        state: Data<&State>,
        batch_id: Query<String>,
    ) -> Result<Json<BatchResponse>> {
        let batch_id = batch_id.0;

        // Return directly if cached.
        let cache_key = format!("batch-{batch_id}");
        if let Some(response) = BatchResponse::from_cache(state.cache.as_ref(), &cache_key).await {
            log::debug!("OpenAPI - Get batch from Cache: {response:?}");
            return Ok(Json(response));
        };

        let block_batch = block_batch_query::fetch_one(&state.db_pool, &batch_id)
            .await
            .map_err(|e| api_err!(e))?;
        let response = BatchResponse::new(block_batch);

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
            |val| if val > 0 { val } else { DEFAULT_PER_PAGE },
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

        let total = block_batch_query::get_total(&state.db_pool)
            .await
            .map_err(|e| api_err!(e))?;
        let block_batches = block_batch_query::fetch_all(&state.db_pool, offset, limit)
            .await
            .map_err(|e| api_err!(e))?;
        let response = BatchesResponse::new(total, block_batches);

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
        batch_id: Query<String>,
    ) -> Result<Json<BlocksResponse>> {
        let batch_id = batch_id.0;

        // Return directly if cached.
        let cache_key = format!("blocks-of-batch-{batch_id}");
        if let Some(response) = BlocksResponse::from_cache(state.cache.as_ref(), &cache_key).await {
            log::debug!("OpenAPI - Get blocks from Cache: {response:?}");
            return Ok(Json(response));
        };

        let block_results = block_result_query::fetch_all(&state.db_pool, &batch_id)
            .await
            .map_err(|e| api_err!(e))?;
        let response = BlocksResponse::new(block_results);

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

        let status_indexes = block_batch_query::get_max_status_indexes(&state.db_pool)
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
}
