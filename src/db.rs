use anyhow::{Context, Result};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

/// Get the connection string to SQLite DB
fn get_db_connection_str() -> Result<String> {
    // Use data directory to standardize storage
    let data_dir = dirs::data_dir()
        .with_context(|| "Unable to find data directory")?
        .join("td");

    // Create directory if it doesn't exist
    std::fs::create_dir_all(&data_dir).with_context(|| "Unable to create data directory")?;

    Ok(format!("sqlite:{}", data_dir.join("todos.db").display()))
}

/// Create connection to SQLite DB pool and create DB if not present
async fn get_db_pool(db_connection_str: &str) -> Result<SqlitePool> {
    // Create connection options
    let opts = SqliteConnectOptions::from_str(db_connection_str)
        .with_context(|| "Failed to create options for DB")?
        .create_if_missing(true);

    // Connect in a pool
    let pool = SqlitePool::connect_with(opts)
        .await
        .with_context(|| "Failed to create DB pool")?;

    Ok(pool)
}
