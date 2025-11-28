//! Server execution module.
//!
//! Provides functions to run an Axum server with database connectivity
//! and optional automatic migrations.
//!
//! ## Functions
//!
//! - [`run`] - Run server without migrations
//! - [`run_with_migrator`] - Run server with optional migrations
//!
//! ## Example
//!
//! ```rust,ignore
//! use sword::server::run_with_migrator;
//! use axum::Router;
//!
//! run_with_migrator::<Migrator, _>(|ctx| {
//!     Router::new().with_state(ctx.clone())
//! }, true).await?;
//! ```

use crate::config::AppConfig;
use crate::db;
use axum::Router;
use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;

/// Shared context available to all route handlers.
///
/// Contains the application configuration and database connection.
/// Clone this to pass it as Axum state.
#[derive(Clone)]
pub struct FrameworkContext {
    /// Application configuration.
    pub config: AppConfig,
    /// Database connection pool.
    pub db: DatabaseConnection,
}

/// Runs the Axum server without database migrations.
///
/// # Arguments
///
/// * `build_router` - A function that receives a [`FrameworkContext`] and returns a configured [`Router`].
///
/// # Example
///
/// ```rust,ignore
/// use sword::server::run;
/// use axum::{Router, routing::get};
///
/// run(|ctx| {
///     Router::new()
///         .route("/health", get(|| async { "OK" }))
///         .with_state(ctx.clone())
/// }).await?;
/// ```
pub async fn run<F>(build_router: F) -> anyhow::Result<()>
where
    F: FnOnce(&FrameworkContext) -> Router,
{
    let config = AppConfig::from_env()?;
    let db = db::connect_db(&config).await?;

    let ctx = FrameworkContext {
        config: config.clone(),
        db,
    };

    let app = build_router(&ctx);

    let bind_addr = config.bind_address();
    tracing::info!("Starting server on {}", bind_addr);

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Runs the Axum server with optional database migrations.
///
/// # Arguments
///
/// * `build_router` - A function that receives a [`FrameworkContext`] and returns a configured [`Router`].
/// * `run_migrations` - If `true`, runs pending migrations before starting the server.
///
/// # Type Parameters
///
/// * `M` - A type implementing [`MigratorTrait`] from SeaORM.
///
/// # Example
///
/// ```rust,ignore
/// use sword::server::run_with_migrator;
/// use axum::{Router, routing::get};
///
/// run_with_migrator::<Migrator, _>(|ctx| {
///     Router::new()
///         .route("/health", get(|| async { "OK" }))
///         .with_state(ctx.clone())
/// }, true).await?;
/// ```
pub async fn run_with_migrator<M, F>(build_router: F, run_migrations: bool) -> anyhow::Result<()>
where
    M: MigratorTrait,
    F: FnOnce(&FrameworkContext) -> Router,
{
    let config = AppConfig::from_env()?;
    let db = db::connect_db(&config).await?;

    if run_migrations {
        tracing::info!("Running database migrations...");
        M::up(&db, None).await?;
        tracing::info!("Migrations completed successfully");
    }

    let ctx = FrameworkContext {
        config: config.clone(),
        db,
    };

    let app = build_router(&ctx);

    let bind_addr = config.bind_address();
    tracing::info!("Starting server on {}", bind_addr);

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
