#![warn(missing_docs)]

//! # SwordAI
//!
//! Making Rust productive for backend, AI systems, data engineering,
//! and distributed systems.
//!
//! Built on [Axum](https://github.com/tokio-rs/axum) and
//! [SeaORM](https://github.com/SeaQL/sea-orm).
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
//! use sword_ai::{server::run_with_migrator, FrameworkContext};
//! use axum::{Router, routing::get};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     dotenvy::dotenv().ok();
//!     sword_ai::tracing::init_tracing();
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
