use anyhow::{Context, Result};
use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

/// Create an in-memory SQLite database for testing
pub async fn setup_test_db() -> Result<SqlitePool> {
    // Connection options as in main implementation
    let opts = SqliteConnectOptions::from_str("sqlite::memory:")
        .with_context(|| "Failed to create options for DB")?
        .create_if_missing(true);

    // Connect in a pool
    let pool = SqlitePool::connect_with(opts)
        .await
        .with_context(|| "Failed to create DB pool")?;

    // Run migrations
    static MIGRATOR: Migrator = sqlx::migrate!();
    MIGRATOR.run(&pool).await?;

    Ok(pool)
}
