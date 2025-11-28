# Sword (`sword-rs`)

**The core library of the Sword ecosystem.**

Part of [Sword](https://github.com/mateusramos/sword) — an ecosystem of libraries and tools designed to bring both **performance** and **productivity** to backend development and distributed systems in Rust.

This library provides batteries-included utilities built on [Axum](https://github.com/tokio-rs/axum) and [SeaORM](https://github.com/SeaQL/sea-orm), letting you focus on business logic instead of boilerplate.

## Features

- **Server Setup**: Pre-configured Axum server with tracing
- **Database**: SeaORM integration with PostgreSQL and connection pooling
- **Migrations**: Automatic database migration support via SeaORM
- **Configuration**: Environment-based configuration with sensible defaults
- **Tracing**: Built-in structured logging with `tracing`

### Roadmap

- 📨 Event-driven architecture (message queues, pub/sub)
- ⚙️ Background jobs and workers
- 🔌 gRPC support
- ☁️ Cloud storage integrations (S3, GCP Storage, Azure Blob)
- 📊 Observability and metrics
- 🔐 Authentication and authorization primitives

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
sword = { package = "sword-rs", version = "0.1" }
```

## Quick Start

```rust
use sword::{server::run_with_migrator, FrameworkContext};
use axum::{Router, routing::get};

// Your migrator (from sea-orm-migration)
struct Migrator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    sword::tracing::init_tracing();

    run_with_migrator::<Migrator, _>(build_router, true).await
}

fn build_router(ctx: &FrameworkContext) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .with_state(ctx.clone())
}
```

## Configuration

The framework reads configuration from environment variables:

| Variable             | Description                       | Default                       |
| -------------------- | --------------------------------- | ----------------------------- |
| `DATABASE_URL`       | PostgreSQL connection string      | **Required**                  |
| `APP_HOST`           | Server bind host                  | `0.0.0.0`                     |
| `APP_PORT`           | Server bind port                  | `3000`                        |
| `RUST_LOG`           | Log level filter                  | `info,sqlx=warn,sea_orm=info` |
| `DB_MAX_CONNECTIONS` | Maximum database connections      | `100`                         |
| `DB_MIN_CONNECTIONS` | Minimum database connections      | `5`                           |
| `DB_CONNECT_TIMEOUT` | Connection timeout (seconds)      | `8`                           |
| `DB_IDLE_TIMEOUT`    | Idle connection timeout (seconds) | `600`                         |
| `DB_MAX_LIFETIME`    | Max connection lifetime (seconds) | `1800`                        |

## Modules

- **`config`** - Application configuration from environment variables
- **`db`** - Database connection with SeaORM
- **`server`** - Axum server setup and execution
- **`tracing`** - Structured logging initialization

## CLI Tool

For quickly scaffolding new projects, download the Sword CLI from [GitHub Releases](https://github.com/mateusramos/sword/releases):

```bash
sword new my-api
```

See the main [repository](https://github.com/mateusramos/sword) for installation instructions.

## License

MIT
