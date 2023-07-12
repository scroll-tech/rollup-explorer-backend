use crate::{cache::Cache, db::DbPool, Settings};
use anyhow::Result;
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use std::sync::Arc;

mod apis;
mod objects;
mod responses;

#[derive(Clone, Debug)]
struct State {
    cache: Arc<Cache>,
    db_pool: DbPool,
    max_per_page: u64,
}

pub async fn run(cache: Arc<Cache>) -> Result<()> {
    let settings = Settings::get();
    let db_pool = DbPool::connect(settings.db_url.as_str()).await?;
    let max_per_page = settings.max_per_page;
    let state = State {
        cache,
        db_pool,
        max_per_page,
    };

    let open_api_addr = &settings.open_api_addr;
    let svr = OpenApiService::new(apis::Apis, "Scroll Rollup Explorer", "2.0")
        .server(format!("{open_api_addr}/api"));

    let ui = svr.swagger_ui();
    let spec = svr.spec();
    let app = Route::new()
        .nest("/", ui)
        .nest("/api", svr)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        // TODO: Fix to only allow specified origins.
        .with(Cors::new().allow_origins_fn(|_| true))
        .data(state);

    let bind_addr = format!("0.0.0.0:{}", settings.bind_port);
    Server::new(TcpListener::bind(bind_addr)).run(app).await?;

    Ok(())
}
