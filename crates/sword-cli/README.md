# Sword CLI

**Scaffold production-ready Rust backend projects in seconds.**

Part of [Sword](https://github.com/mateusramos/sword) — an ecosystem of libraries and tools designed to bring both **performance** and **productivity** to backend development and distributed systems in Rust.

This CLI generates opinionated, well-structured projects with sensible defaults so you can focus on what matters: your business logic.

## Installation

### Download the binary

Download the latest release for your platform from [GitHub Releases](https://github.com/mateusramos/sword/releases).

```bash
# Linux/macOS - move to PATH
chmod +x sword
sudo mv sword /usr/local/bin/

# Or add to your local bin
mv sword ~/.local/bin/
```

### Build from source

```bash
git clone https://github.com/mateusramos/sword.git
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

# Initialize in current directory
sword new --init
```

### Command Options

| Flag               | Description                                                        |
| ------------------ | ------------------------------------------------------------------ |
| `--out-dir`, `-o`  | Output directory (default: `..`)                                   |
| `--init`           | Initialize in specified directory instead of creating subdirectory |
| `--no-interactive` | Disable interactive prompts                                        |
| `--port`, `-p`     | Application port (default: `3000`)                                 |

## Generated Project

The CLI generates a complete backend project with:

- **Axum** web server
- **SeaORM** database integration with PostgreSQL
- **Clean architecture** (app/domain/infrastructure layers)
- **Docker Compose** setup for PostgreSQL
- **Dockerfile** for production deployment
- **Automatic migrations** on startup

See the main [repository](https://github.com/mateusramos/sword) for more details.

## License

MIT
