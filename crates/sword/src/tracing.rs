//! Tracing initialization module.
//!
//! Provides [`init_tracing`] for setting up structured logging with
//! the `tracing` ecosystem.
//!
//! ## Log Levels
//!
//! Default filter: `info,sqlx=warn,sea_orm=info`
//!
//! Override with the `RUST_LOG` environment variable:
//!
//! ```bash
//! RUST_LOG=debug cargo run
//! ```
//!
//! ## Example
//!
//! ```rust,ignore
//! use sword::tracing::init_tracing;
//!
//! fn main() {
//!     init_tracing();
//!     tracing::info!("Application started");
//! }
//! ```

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initializes the global tracing subscriber.
///
/// Call this once at the start of your application, before any logging.
/// Reads log level configuration from the `RUST_LOG` environment variable.
pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sqlx=warn,sea_orm=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
