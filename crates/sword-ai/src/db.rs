//! Database connection module.
//!
//! Provides [`connect_db`] for establishing a connection pool to PostgreSQL
//! using SeaORM with configurable settings.
//!
//! ## Connection Pool Settings
//!
//! Configured via environment variables (see [`crate::config::AppConfig`]):
//!
//! - `DB_MAX_CONNECTIONS`: Maximum connections (default: 100)
//! - `DB_MIN_CONNECTIONS`: Minimum connections (default: 5)
//! - `DB_CONNECT_TIMEOUT`: Connection timeout in seconds (default: 8)
//! - `DB_IDLE_TIMEOUT`: Idle timeout in seconds (default: 600)
//! - `DB_MAX_LIFETIME`: Max connection lifetime in seconds (default: 1800)
//!
//! ## Example
//!
//! ```rust,ignore
//! use sword_ai::{connect_db, AppConfig};
//!
//! let config = AppConfig::from_env()?;
//! let db = connect_db(&config).await?;
//! ```

use crate::config::AppConfig;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

/// Establishes a database connection with a configured connection pool.
///
/// # Arguments
///
/// * `config` - Application configuration containing database settings
///
/// # Returns
///
/// A [`DatabaseConnection`] ready for use with SeaORM queries.
pub async fn connect_db(config: &AppConfig) -> anyhow::Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(config.database_url.clone());
    opt.max_connections(config.db_max_connections)
        .min_connections(config.db_min_connections)
        .connect_timeout(Duration::from_secs(config.db_connect_timeout))
        .acquire_timeout(Duration::from_secs(config.db_connect_timeout))
        .idle_timeout(Duration::from_secs(config.db_idle_timeout))
        .max_lifetime(Duration::from_secs(config.db_max_lifetime))
        .sqlx_logging(true)
        .sqlx_logging_level(tracing::log::LevelFilter::Info);

    let db = Database::connect(opt).await?;

    tracing::info!("Database connection established");

    Ok(db)
}
