use crate::Settings;
use anyhow::Result;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPool;

static MIGRATOR: Migrator = sqlx::migrate!("db/migrations");

pub async fn run() -> Result<()> {
    let db_pool = PgPool::connect(Settings::get().db_url.as_str()).await?;
    MIGRATOR.run(&db_pool).await?;
    Ok(())
}
