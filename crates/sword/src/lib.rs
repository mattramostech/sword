#![warn(missing_docs)]

//! # Sword Framework
//!
//! A backend framework built on [Axum](https://github.com/tokio-rs/axum) and
//! [SeaORM](https://github.com/SeaQL/sea-orm) for rapid API development.
//!
//! ## Features
//!
//! - Pre-configured Axum server with tracing
//! - SeaORM integration with PostgreSQL
//! - Automatic database migration support
//! - Environment-based configuration
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use sword::{server::run_with_migrator, FrameworkContext};
//! use axum::{Router, routing::get};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     dotenvy::dotenv().ok();
//!     sword::tracing::init_tracing();
//!
//!     run_with_migrator::<Migrator, _>(build_router, true).await
//! }
//!
//! fn build_router(ctx: &FrameworkContext) -> Router {
//!     Router::new()
//!         .route("/health", get(|| async { "OK" }))
//!         .with_state(ctx.clone())
//! }
//! ```

pub mod config;
pub mod db;
pub mod server;
pub mod tracing;

pub use config::AppConfig;
pub use db::connect_db;
pub use server::FrameworkContext;
