# Build stage
FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/app

# Pre-build dependencies caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -r src

# Actual source code
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Required for SQLx with Postgres
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/axum-web /app/axum-web

EXPOSE 3000
CMD ["./axum-web"]
