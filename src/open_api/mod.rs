use crate::{cache::Cache, db::DbPool, Settings};
use anyhow::Result;
use futures::join;
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
}

pub async fn run(cache: Arc<Cache>) -> Result<()> {
    let settings = Settings::get();
    let db_pool = PgPoolOptions::new()
        .max_connections(settings.max_db_conns)
        .connect(settings.db_url.as_str())
        .await?;

    let state = State { cache, db_pool };

    let open_api_addr = &settings.open_api_addr;
    let svr = OpenApiService::new(apis::Apis, "Scroll Rollup Explorer", "2.0")
        .server(format!("{open_api_addr}/api"));

    let tokio_metrics = TokioMetrics::new();
    let ui = svr.swagger_ui();
    let spec = svr.spec();

    let metrics = Route::new()
        .at("/metrics", PrometheusExporter::new(prometheus_registry()))
        .at("/tokio_metrics", tokio_metrics.exporter())
        // TODO: Fix to only allow specified origins.
        .with(Cors::new().allow_origins_fn(|_| true))
        .with(Tracing);

    let app = Route::new()
        .nest("/", ui)
        .nest("/api", svr.with(tokio_metrics))
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        // TODO: Fix to only allow specified origins.
        .with(Cors::new().allow_origins_fn(|_| true))
        .data(state);

    let app_bind_addr = format!("0.0.0.0:{}", settings.bind_port);
    let metrics_bind_addr = format!("0.0.0.0:{}", settings.metrics_bind_port);
    let app_server = Server::new(TcpListener::bind(app_bind_addr)).run(app);
    let metrics_server = Server::new(TcpListener::bind(metrics_bind_addr)).run(metrics);

    let (app_result, metrics_result) = join!(app_server, metrics_server);
    if let Err(err) = app_result.and(metrics_result) {
        log::error!("An error occurred in run function: {err}");
    }

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
