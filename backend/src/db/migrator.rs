use crate::Settings;
use anyhow::Result;
use sqlx::migrate::{MigrateDatabase, Migrator};
use sqlx::postgres::{PgPool, Postgres};

static MIGRATOR: Migrator = sqlx::migrate!("db/migrations");

pub async fn run() -> Result<()> {
    // Create database if non existing.
    let db_url = &Settings::get().db_url.as_str();
    if !Postgres::database_exists(db_url).await? {
        Postgres::create_database(db_url).await?;
    }

    // Run migrations.
    let db_pool = PgPool::connect(db_url).await?;
    MIGRATOR.run(&db_pool).await?;
    Ok(())
}
