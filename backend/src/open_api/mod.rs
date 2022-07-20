use crate::cache::Cache;
use crate::db::DbPool;
use crate::Settings;
use anyhow::Result;
use poem::listener::TcpListener;
use poem::middleware::Cors;
use poem::{EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use std::sync::Arc;

mod apis;
mod objects;

#[derive(Clone, Debug)]
struct State {
    cache: Arc<Cache>,
    db_pool: DbPool,
}

pub async fn run(cache: Arc<Cache>) -> Result<()> {
    let settings = Settings::get();
    let db_pool = DbPool::connect(settings.db_url.as_str()).await?;
    let state = State { cache, db_pool };

    let open_api_addr = &settings.open_api_addr;
    let svr = OpenApiService::new(apis::Apis, "Scroll L2 Explorer", "1.0")
        .server(format!("http://{open_api_addr}/api"));

    let ui = svr.swagger_ui();
    let spec = svr.spec();
    let app = Route::new()
        .nest("/", ui)
        .nest("/api", svr)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(Cors::new())
        .data(state);

    Server::new(TcpListener::bind("0.0.0.0:5001"))
        .run(app)
        .await?;

    Ok(())
}
