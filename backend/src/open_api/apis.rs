use crate::db::{l2_block_query, tps_query, DbPool};
use crate::open_api::objects::L2Block;
use poem::error::InternalServerError;
use poem::web::Data;
use poem::Result;
use poem_openapi::param::Query;
use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};
use rust_decimal::Decimal;

pub(crate) struct Apis;

// Query parameter `page` starts from `1`, and default `per_page` is 20.
const DEFAULT_PER_PAGE: u64 = 20;

#[OpenApi]
impl Apis {
    #[oai(path = "/l1_tps", method = "get")]
    async fn l1_tps(&self, db_pool: Data<&DbPool>) -> Result<Json<TpsResponse>> {
        let tps = tps_query::get_l1_tps(&db_pool)
            .await
            .map_err(InternalServerError)?;
        // Only keep two decimal digits.
        Ok(Json(TpsResponse {
            tps: tps.round_dp(2),
        }))
    }

    #[oai(path = "/l2_blocks", method = "get")]
    async fn l2_blocks(
        &self,
        db_pool: Data<&DbPool>,
        page: Query<Option<u64>>,
        per_page: Query<Option<u64>>,
    ) -> Result<Json<L2BlocksResponse>> {
        let total = l2_block_query::get_total(&db_pool)
            .await
            .map_err(InternalServerError)?;

        let limit = per_page.0.map_or_else(
            || DEFAULT_PER_PAGE,
            |val| if val > 0 { val } else { DEFAULT_PER_PAGE },
        );
        let offset = page
            .0
            .map_or_else(|| 0, |val| if val > 0 { (val - 1) * limit } else { 0 });
        let blocks = l2_block_query::fetch_all(&db_pool, offset, limit)
            .await
            .map_err(InternalServerError)?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(Json(L2BlocksResponse { total, blocks }))
    }

    #[oai(path = "/l2_tps", method = "get")]
    async fn l2_tps(&self, db_pool: Data<&DbPool>) -> Result<Json<TpsResponse>> {
        let tps = tps_query::get_l2_tps(&db_pool)
            .await
            .map_err(InternalServerError)?;
        // Only keep two decimal digits.
        Ok(Json(TpsResponse {
            tps: tps.round_dp(2),
        }))
    }
}

#[derive(Object)]
struct L2BlocksResponse {
    total: i64,
    blocks: Vec<L2Block>,
}

#[derive(Object)]
struct TpsResponse {
    tps: Decimal,
}
