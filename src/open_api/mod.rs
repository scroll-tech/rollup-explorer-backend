use crate::{cache::Cache, db::DbPool, Settings};
use anyhow::Result;
use lazy_static::lazy_static;
use poem::{
    endpoint::PrometheusExporter,
    listener::TcpListener,
    middleware::{Cors, TokioMetrics, Tracing},
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use prometheus::{HistogramOpts, HistogramVec, IntCounter, IntCounterVec, Opts, Registry};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

mod apis;
mod objects;
mod responses;

lazy_static! {
    pub static ref INCOMING_REQUESTS: IntCounter =
        IntCounter::new("incoming_requests", "Incoming Requests").unwrap();
    pub static ref RESPONSE_TIME_COLLECTOR: HistogramVec = HistogramVec::new(
        HistogramOpts::new("response_time", "Response Times"),
        &["api"]
    )
    .unwrap();
    pub static ref CACHE_HITS: IntCounterVec =
        IntCounterVec::new(Opts::new("cache_hits", "Cache Hits"), &["cache"]).unwrap();
}

#[derive(Clone, Debug)]
struct State {
    cache: Arc<Cache>,
    db_pool: DbPool,
    max_per_page: u64,
}

pub async fn run(cache: Arc<Cache>) -> Result<()> {
    let settings = Settings::get();
    let db_pool = PgPoolOptions::new()
        .max_connections(settings.max_db_conns)
        .connect(settings.db_url.as_str())
        .await?;

    let max_per_page = settings.max_per_page;
    let state = State {
        cache,
        db_pool,
        max_per_page,
    };

    let open_api_addr = &settings.open_api_addr;
    let svr = OpenApiService::new(apis::Apis, "Scroll Rollup Explorer", "2.0")
        .server(format!("{open_api_addr}/api"));

    let tokio_metrics = TokioMetrics::new();
    let ui = svr.swagger_ui();
    let spec = svr.spec();
    let app = Route::new()
        .nest("/", ui)
        .at("/tokio_metrics", tokio_metrics.exporter())
        .nest("/api", svr.with(tokio_metrics))
        .at("/metrics", PrometheusExporter::new(prometheus_registry()))
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        // TODO: Fix to only allow specified origins.
        .with(Cors::new().allow_origins_fn(|_| true))
        .with(Tracing)
        .data(state);

    let bind_addr = format!("0.0.0.0:{}", settings.bind_port);
    Server::new(TcpListener::bind(bind_addr)).run(app).await?;

    Ok(())
}

fn prometheus_registry() -> Registry {
    let registry = Registry::new();
    registry
        .register(Box::new(INCOMING_REQUESTS.clone()))
        .unwrap();

    registry
        .register(Box::new(RESPONSE_TIME_COLLECTOR.clone()))
        .unwrap();
    registry.register(Box::new(CACHE_HITS.clone())).unwrap();

    registry
}
