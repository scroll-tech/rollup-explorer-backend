use crate::cache::Cache;
use crate::db::{l2_block_query, tps_query};
use crate::open_api::objects::L2Block;
use crate::open_api::State;
use poem::error::InternalServerError;
use poem::web::Data;
use poem::Result;
use poem_openapi::param::Query;
use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};
use rust_decimal::Decimal;
use std::sync::Arc;

// Expired seconds of cache data.
const L2_BLOCKS_CACHE_EXPIRED_SECS: u64 = 1;
const TPS_CACHE_EXPIRED_SECS: u64 = 10;
// Query parameter `page` starts from `1`, and default `per_page` is 20.
const DEFAULT_PER_PAGE: u64 = 20;

pub(crate) struct Apis;

#[OpenApi]
impl Apis {
    #[oai(path = "/l1_tps", method = "get")]
    async fn l1_tps(&self, state: Data<&State>) -> Result<Json<TpsResponse>> {
        // Return directly if cached.
        if let Some(response) = get_tps_from_cache(state.cache.as_ref(), "l1_tps").await {
            log::debug!("OpenAPI - Get L1 TPS from Cache: {response:?}");
            return Ok(Json(response));
        };

        let tps = tps_query::get_l1_tps(&state.db_pool)
            .await
            .map_err(InternalServerError)?;

        // Only keep two decimal digits.
        let response = TpsResponse {
            tps: tps.round_dp(2),
        };

        // Save to cache.
        if let Err(error) = state
            .cache
            .set("l1_tps", Arc::new(response.clone()), TPS_CACHE_EXPIRED_SECS)
            .await
        {
            log::error!("OpenAPI - Failed to save cache for L1 TPS: {error}");
        }

        Ok(Json(response))
    }

    #[oai(path = "/l2_blocks", method = "get")]
    async fn l2_blocks(
        &self,
        state: Data<&State>,
        page: Query<Option<u64>>,
        per_page: Query<Option<u64>>,
    ) -> Result<Json<L2BlocksResponse>> {
        let limit = per_page.0.map_or_else(
            || DEFAULT_PER_PAGE,
            |val| if val > 0 { val } else { DEFAULT_PER_PAGE },
        );
        let offset = page
            .0
            .map_or_else(|| 0, |val| if val > 0 { (val - 1) * limit } else { 0 });

        // Return directly if cached.
        let cache_key = format!("l2_block-{offset}-{limit}");
        if let Some(response) = state
            .cache
            .get(&cache_key)
            .await
            .ok()
            .flatten()
            .and_then(|any| any.downcast_ref::<L2BlocksResponse>().cloned())
        {
            log::debug!("OpenAPI - Get L2 blocks from Cache: {response:?}");
            return Ok(Json(response));
        };

        let total = l2_block_query::get_total(&state.db_pool)
            .await
            .map_err(InternalServerError)?;

        let blocks = l2_block_query::fetch_all(&state.db_pool, offset, limit)
            .await
            .map_err(InternalServerError)?
            .into_iter()
            .map(Into::into)
            .collect();

        let response = L2BlocksResponse { total, blocks };

        // Save to cache.
        if let Err(error) = state
            .cache
            .set(
                &cache_key,
                Arc::new(response.clone()),
                L2_BLOCKS_CACHE_EXPIRED_SECS,
            )
            .await
        {
            log::error!("OpenAPI - Failed to save cache for L2 blocks: {error}");
        }

        Ok(Json(response))
    }

    #[oai(path = "/l2_tps", method = "get")]
    async fn l2_tps(&self, state: Data<&State>) -> Result<Json<TpsResponse>> {
        // Return directly if cached.
        if let Some(response) = get_tps_from_cache(state.cache.as_ref(), "l2_tps").await {
            log::debug!("OpenAPI - Get L2 TPS from Cache: {response:?}");
            return Ok(Json(response));
        };

        let tps = tps_query::get_l2_tps(&state.db_pool)
            .await
            .map_err(InternalServerError)?;

        // Only keep two decimal digits.
        let response = TpsResponse {
            tps: tps.round_dp(2),
        };

        // Save to cache.
        if let Err(error) = state
            .cache
            .set("l2_tps", Arc::new(response.clone()), TPS_CACHE_EXPIRED_SECS)
            .await
        {
            log::error!("OpenAPI - Failed to save cache for L2 TPS: {error}");
        }

        Ok(Json(response))
    }
}

#[derive(Clone, Debug, Object)]
struct L2BlocksResponse {
    total: i64,
    blocks: Vec<L2Block>,
}

#[derive(Clone, Debug, Object)]
struct TpsResponse {
    tps: Decimal,
}

async fn get_tps_from_cache(cache: &Cache, cache_key: &str) -> Option<TpsResponse> {
    cache
        .get(cache_key)
        .await
        .ok()
        .flatten()
        .and_then(|any| any.downcast_ref::<TpsResponse>().cloned())
}
