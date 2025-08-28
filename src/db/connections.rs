use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;


// Default variables
const DEFAULT_DB_NAME: &str = "dojo";
const DEFAULT_DB_FILE: &str = "judo.db";

/// Config file definition
#[derive(Deserialize, Serialize)]
pub struct ConfigFile {
    default: String,
    dbs: Vec<DBConfig>,
}

/// Database configuration
#[derive(Deserialize, Serialize)]
pub struct DBConfig {
    name: String,
    path: String,
}

impl Default for DBConfig {
    fn default() -> Self {
        // Use data directory to standardize storage
        let data_dir = dirs::data_dir().unwrap().join("judo");

        // Create directory
        std::fs::create_dir_all(&data_dir).unwrap();

        // Create path to db
        let path = data_dir.join(DEFAULT_DB_FILE).to_str().unwrap().to_string();

        Self {
            name: DEFAULT_DB_NAME.to_string(),
            path: path,
        }
    }
}

impl Default for ConfigFile {

    /// By default, the name is the default name with default config
    fn default() -> Self {
        Self {
            default: DEFAULT_DB_NAME.to_string(),
            dbs: vec![DBConfig::default()]
        }
    }
}

// impl DBConfig {

//     /// Create new database. Only name is necessary, path is created if missing by default
//     pub fn new(name: String, path: Option<String>) -> Result<Self> {
//         if None(path) {

//             // Use data directory to standardize storage
//             let data_dir = dirs::data_dir().unwrap().join("judo");

//             // Create directory
//             std::fs::create_dir_all(&data_dir).unwrap();

//             // Create path to db
//             let db_path = data_dir.join(format!("{}.db", name));

//             if db_path.exists(){
//                 todo!()
//             }

//             Ok(Self {
//                 name: name, 
//                 path: db_path.to_str().unwrap().to_string()
//             })

//         } else {

//             let db_path = Path::from(path);

//             if !db_path.exists(){
//                 todo!()
//             }


//             Ok(Self {
//                 name: name,
//                 path: db_path
//             })
//         }
//     }
// }

fn get_db_connection_str() -> Result<String> {
    // Use data directory to standardize storage
    let data_dir = dirs::data_dir()
        .with_context(|| "Unable to find data directory")?
        .join("judo");

    // Create directory
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

/// Run database migrations
async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    // Embed the migration files into binary
    static MIGRATOR: Migrator = sqlx::migrate!();

    MIGRATOR
        .run(pool)
        .await
        .with_context(|| "Failed to run database migrations")?;

    Ok(())
}

/// Initialize database with connection and run migrations
/// This is safe to call on every startup - migrations are idempotent
pub async fn init_db() -> Result<SqlitePool> {
    let connection_string = get_db_connection_str()?;
    let pool = get_db_pool(&connection_string).await?;

    // Always run migrations on startup - they're idempotent and fast
    run_migrations(&pool).await?;

    Ok(pool)
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_get_db_connection_str_success() -> Result<()> {
        let _db_connection = get_db_connection_str()?;
        Ok(())
    }

    #[tokio::test]
    async fn test_connection_pool_success() -> Result<()> {
        let connection_string = get_db_connection_str()?;
        let _pool = get_db_pool(&connection_string).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_connection_pool_in_memory_success() -> Result<()> {
        let connection_string = "sqlite::memory:".to_string();
        let _pool = get_db_pool(&connection_string).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_migrations() -> Result<()> {
        let connection_string = "sqlite::memory:".to_string();
        let pool = get_db_pool(&connection_string).await?;
        run_migrations(&pool).await?;
        Ok(())
    }
}
