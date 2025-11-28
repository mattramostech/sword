# SwordAI

**Making Rust productive for backend, AI systems, data engineering, and distributed systems.**

SwordAI is more than a framework — it's a growing ecosystem of libraries and tools designed to bring both **performance** and **productivity** to backend development, AI systems, data engineering, and distributed systems in Rust.

## Vision

Rust offers unmatched performance and safety, but building production systems often requires stitching together many crates with significant boilerplate. SwordAI aims to change that by providing:

- **Batteries-included libraries** for common backend, distributed systems, AI, and data engineering patterns
- **A CLI** that scaffolds production-ready projects in seconds
- **Opinionated defaults** that let you focus on business logic

### Current Features

- 🚀 REST APIs with [Axum](https://github.com/tokio-rs/axum)
- 🗄️ PostgreSQL with [SeaORM](https://github.com/SeaQL/sea-orm)
- 📦 Clean architecture scaffolding (controllers, services, repositories)
- 🐳 Docker-ready setup out of the box

### Roadmap

- 🤖 AI/ML integration primitives
- 📨 Event-driven architecture (message queues, pub/sub)
- ⚙️ Background jobs and workers
- 🔌 gRPC support
- ☁️ Cloud storage integrations (S3, GCP Storage, Azure Blob)
- 📊 Observability and metrics
- 🔐 Authentication and authorization primitives
- 🔄 Distributed systems patterns

## Structure

This repository contains:

- `crates/sword-ai` - The SwordAI framework library with server configuration, database connection, and tracing utilities
- `crates/sword-cli` - CLI tool to generate new projects using the SwordAI framework

## Installation

### Download the CLI binary

Download the latest release for your platform from [GitHub Releases](https://github.com/mattramostech/sword-ai/releases).

```bash
# Linux/macOS - move to PATH
chmod +x sword
sudo mv sword /usr/local/bin/

# Or add to your local bin
mv sword ~/.local/bin/
```

### Build from source

```bash
git clone https://github.com/mattramostech/sword-ai.git
cd sword
cargo build --release
```

The binary will be available at `target/release/sword`.

## Usage

### Creating a new project

```bash
# Interactive mode (will prompt for project name)
sword new

# With project name
sword new my-api

# Specify output directory (defaults to parent directory)
sword new my-api --out-dir /path/to/projects

# Non-interactive mode
sword new my-api --no-interactive
```

### Generated Project Structure

The generated project includes:

```
my-api/
├── src/
│   ├── main.rs
│   ├── app/
│   │   ├── mod.rs
│   │   ├── routes.rs
│   │   └── controllers/
│   │       ├── mod.rs
│   │       └── users_controller.rs
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── entities/
│   │   │   ├── mod.rs
│   │   │   └── user.rs
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   └── user_service.rs
│   │   └── repositories/
│   │       ├── mod.rs
│   │       └── user_repository.rs
│   └── infrastructure/
│       ├── mod.rs
│       └── database/
│           ├── mod.rs
│           ├── models/
│           │   ├── mod.rs
│           │   └── users.rs
│           └── migration/
│               ├── mod.rs
│               └── m20220101_000001_create_user.rs
├── Cargo.toml
├── Dockerfile
├── docker-compose.yml
├── .env
└── .gitignore
```

### Running the Generated Project

1. Navigate to your project:

   ```bash
   cd my-api
   ```

2. Start PostgreSQL:

   ```bash
   docker compose up -d db
   ```

3. Run the application (migrations run automatically):

   ```bash
   cargo run
   ```

4. Test the endpoints:

   ```bash
   # Create a user
   curl -X POST http://localhost:3000/users \
     -H "Content-Type: application/json" \
     -d '{"name": "John Doe", "email": "john@example.com"}'

   # Get a user
   curl http://localhost:3000/users/1
   ```

5. Run integration tests:
   ```bash
   cargo test
   ```

## Features

The SwordAI framework provides:

- **Server Setup**: Pre-configured Axum server with tracing
- **Database**: SeaORM integration with PostgreSQL
- **Migrations**: Automatic database migration support
- **Project Structure**: Clean architecture with separation of concerns
  - `app/` - HTTP controllers and routes
  - `domain/` - Business logic, entities, services, and repository traits
  - `infrastructure/` - Database implementations and migrations
  - `tests/` - Integration tests

### Example Endpoints

Generated projects include example user management endpoints:

- `POST /users` - Create a new user
- `GET /users/:id` - Get a user by ID

## Development

To work on the SwordAI framework itself:

```bash
# Build the workspace
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Run clippy
cargo clippy
```

## Release Process

When releasing a new version:

1. Update `sword-ai` crate version in `crates/sword-ai/Cargo.toml`
2. Run `cargo publish -p sword-ai`
3. Update `SWORD_VERSION` constant in `crates/sword-cli/src/commands/new.rs`
4. Build CLI binaries for each platform and create a GitHub Release

## Contributing

When contributing, if you modify the `sword-ai` library in a way that requires a version bump:

1. Update the version in `crates/sword-ai/Cargo.toml`
2. Update `SWORD_VERSION` in `crates/sword-cli/src/commands/new.rs` to match

## License

MIT
