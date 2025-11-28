FROM rust:1.83-bookworm as builder

WORKDIR /app

# Copy manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy src to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source code
COPY src ./src

# Build the real application
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/{{PROJECT_NAME}} /app/{{PROJECT_NAME}}

ENV APP_HOST=0.0.0.0
ENV APP_PORT={{APP_PORT}}

EXPOSE {{APP_PORT}}

CMD ["./{{PROJECT_NAME}}"]
