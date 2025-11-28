# Sword Framework - Quick Start Guide

This guide will help you get started with the Sword CLI and create your first backend project.

## Prerequisites

- Rust 1.75+ (see `rust-version` in `Cargo.toml` for MSRV)
- Docker and Docker Compose
- PostgreSQL client (optional, for manual database access)

## Step 1: Install the Sword CLI

Download the latest release for your platform from [GitHub Releases](https://github.com/mateusramos/sword-ai/releases):

```bash
# Linux/macOS
chmod +x sword
sudo mv sword /usr/local/bin/
```

Or build from source:

```bash
git clone https://github.com/mateusramos/sword-ai.git
cd sword
cargo build --release
# The binary will be at target/release/sword
```

## Step 2: Create a new project

```bash
sword new my-api --out-dir .
```

Or use interactive mode:

```bash
sword new --out-dir .
# You'll be prompted for the project name and port
```

## Step 3: Start the database

```bash
cd my-api
docker compose up -d db
```

Wait a few seconds for PostgreSQL to be ready. You can check with:

```bash
docker compose ps
```

## Step 4: Run your application

```bash
cargo run
```

The application will:

1. Connect to the database
2. Run migrations automatically
3. Start the HTTP server on `http://0.0.0.0:3000`

## Step 5: Test the API

Create a user:

```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com"}'
```

Get a user:

```bash
curl http://localhost:3000/users/1
```

## Step 6: Run tests

The generated project includes integration tests:

```bash
cargo test
```

## Project Structure

```
my-api/
├── src/
│   ├── main.rs                           # Application entry point
│   ├── app/                              # HTTP layer
│   │   ├── mod.rs
│   │   ├── routes.rs                     # Route definitions
│   │   └── controllers/
│   │       ├── mod.rs
│   │       └── users_controller.rs       # HTTP handlers
│   ├── domain/                           # Business logic
│   │   ├── mod.rs
│   │   ├── entities/
│   │   │   ├── mod.rs
│   │   │   └── user.rs                   # Domain entities
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   └── user_service.rs           # Business services
│   │   └── repositories/
│   │       ├── mod.rs
│   │       └── user_repository.rs        # Repository trait + implementation
│   └── infrastructure/                   # Technical implementations
│       ├── mod.rs
│       └── database/
│           ├── mod.rs
│           ├── models/
│           │   ├── mod.rs
│           │   └── users.rs              # SeaORM entity
│           └── migration/
│               ├── mod.rs                # Migrator setup
│               └── m20220101_000001_create_user.rs # Migration
├── Cargo.toml
├── Dockerfile
├── docker-compose.yml
└── .env                                  # Environment variables
```

## Next Steps

### Add a new endpoint

1. Create a new migration in `src/infrastructure/migration/`
2. Define the entity in `src/domain/entities/`
3. Create repository trait in `src/domain/repositories/`
4. Implement repository in `src/infrastructure/repositories/`
5. Create service in `src/domain/services/`
6. Add controller in `src/app/controllers/`
7. Register routes in `src/app/routes.rs`

### Environment Variables

Edit `.env` to configure:

- `DATABASE_URL`: PostgreSQL connection string
- `APP_HOST`: Server host (default: `0.0.0.0`)
- `APP_PORT`: Server port (default: `3000`)
- `RUST_LOG`: Logging level (default: `info,sqlx=warn,sea_orm=info`)
- `DB_MAX_CONNECTIONS`: Maximum database connections (default: `100`)
- `DB_MIN_CONNECTIONS`: Minimum database connections (default: `5`)
- `DB_CONNECT_TIMEOUT`: Connection timeout in seconds (default: `8`)
- `DB_IDLE_TIMEOUT`: Idle connection timeout in seconds (default: `600`)
- `DB_MAX_LIFETIME`: Max connection lifetime in seconds (default: `1800`)

### Docker Deployment

Build the Docker image:

```bash
docker build -t my-api .
```

Run with docker-compose (add your app service to `docker-compose.yml`).

## Troubleshooting

### Port 5432 already in use

If you already have PostgreSQL running locally:

```bash
# Stop local PostgreSQL
brew services stop postgresql  # macOS
# or
sudo systemctl stop postgresql  # Linux

# Or change the port in docker-compose.yml and .env
```

### Database connection refused

Wait for PostgreSQL to be fully ready:

```bash
docker compose logs db
# Look for "database system is ready to accept connections"
```

## Sword Library Version

Generated projects depend on the `sword-rs` crate from [crates.io](https://crates.io/crates/sword-rs). The version is determined by the CLI at project generation time.

> **Note:** The crate is published as `sword-rs` on crates.io, but you use it as `sword` in your code via the `package` alias.

To update to a newer version, edit your project's `Cargo.toml`:

```toml
[dependencies]
sword = { package = "sword-rs", version = "0.2" }  # or whatever the latest version is
```

## Support

For issues or questions, check the main [README.md](README.md) or open an issue on GitHub.
