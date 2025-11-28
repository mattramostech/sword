//! Application configuration module.
//!
//! Provides [`AppConfig`] for loading configuration from environment variables.
//!
//! ## Environment Variables
//!
//! | Variable | Description | Default |
//! |----------|-------------|---------|
//! | `DATABASE_URL` | PostgreSQL connection string | **Required** |
//! | `APP_HOST` | Server bind host | `0.0.0.0` |
//! | `APP_PORT` | Server bind port | `3000` |
//! | `DB_MAX_CONNECTIONS` | Maximum database connections | `100` |
//! | `DB_MIN_CONNECTIONS` | Minimum database connections | `5` |
//! | `DB_CONNECT_TIMEOUT` | Connection timeout in seconds | `8` |
//! | `DB_IDLE_TIMEOUT` | Idle connection timeout in seconds | `600` |
//! | `DB_MAX_LIFETIME` | Maximum connection lifetime in seconds | `1800` |
//!
//! ## Example
//!
//! ```rust,ignore
//! use sword_ai::AppConfig;
//!
//! let config = AppConfig::from_env()?;
//! println!("Binding to {}", config.bind_address());
//! ```

use std::env;

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Server bind host (from `APP_HOST`, default: `0.0.0.0`).
    pub host: String,
    /// Server bind port (from `APP_PORT`, default: `3000`).
    pub port: u16,
    /// PostgreSQL connection string (from `DATABASE_URL`, required).
    pub database_url: String,
    /// Maximum database connections (from `DB_MAX_CONNECTIONS`, default: `100`).
    pub db_max_connections: u32,
    /// Minimum database connections (from `DB_MIN_CONNECTIONS`, default: `5`).
    pub db_min_connections: u32,
    /// Connection timeout in seconds (from `DB_CONNECT_TIMEOUT`, default: `8`).
    pub db_connect_timeout: u64,
    /// Idle connection timeout in seconds (from `DB_IDLE_TIMEOUT`, default: `600`).
    pub db_idle_timeout: u64,
    /// Maximum connection lifetime in seconds (from `DB_MAX_LIFETIME`, default: `1800`).
    pub db_max_lifetime: u64,
}

impl AppConfig {
    /// Loads configuration from environment variables.
    ///
    /// # Errors
    ///
    /// Returns an error if `DATABASE_URL` is not set or if any numeric
    /// environment variable cannot be parsed.
    pub fn from_env() -> anyhow::Result<Self> {
        let host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("APP_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()?;
        let database_url =
            env::var("DATABASE_URL").map_err(|_| anyhow::anyhow!("DATABASE_URL must be set"))?;

        let db_max_connections = env::var("DB_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "100".to_string())
            .parse::<u32>()?;
        let db_min_connections = env::var("DB_MIN_CONNECTIONS")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u32>()?;
        let db_connect_timeout = env::var("DB_CONNECT_TIMEOUT")
            .unwrap_or_else(|_| "8".to_string())
            .parse::<u64>()?;
        let db_idle_timeout = env::var("DB_IDLE_TIMEOUT")
            .unwrap_or_else(|_| "600".to_string())
            .parse::<u64>()?;
        let db_max_lifetime = env::var("DB_MAX_LIFETIME")
            .unwrap_or_else(|_| "1800".to_string())
            .parse::<u64>()?;

        Ok(Self {
            host,
            port,
            database_url,
            db_max_connections,
            db_min_connections,
            db_connect_timeout,
            db_idle_timeout,
            db_max_lifetime,
        })
    }

    /// Returns the server bind address in `host:port` format.
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> AppConfig {
        AppConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            database_url: "postgres://localhost/test".to_string(),
            db_max_connections: 100,
            db_min_connections: 5,
            db_connect_timeout: 8,
            db_idle_timeout: 600,
            db_max_lifetime: 1800,
        }
    }

    #[test]
    fn test_bind_address() {
        let config = default_config();
        assert_eq!(config.bind_address(), "127.0.0.1:8080");
    }

    #[test]
    fn test_bind_address_default_format() {
        let mut config = default_config();
        config.host = "0.0.0.0".to_string();
        config.port = 3000;
        assert_eq!(config.bind_address(), "0.0.0.0:3000");
    }

    #[test]
    fn test_config_clone() {
        let config = default_config();
        let cloned = config.clone();
        assert_eq!(config.host, cloned.host);
        assert_eq!(config.port, cloned.port);
        assert_eq!(config.database_url, cloned.database_url);
        assert_eq!(config.db_max_connections, cloned.db_max_connections);
    }
}
