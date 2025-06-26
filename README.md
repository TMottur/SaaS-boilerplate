# Axum Web API

A lightweight, production-ready Rust API built with [Axum](https://docs.rs/axum/latest/axum/), designed for real-world backend engineering. This project demonstrates clean API architecture, Postgres-backed session authentication, rate limiting, Dockerized deployment, and robust error handling.

## Features

- **Full CRUD Resources**
  - Manage user accounts and project resources via RESTful endpoints.
  - Relational integrity enforced with PostgreSQL.

- **Secure Authentication**
  - Email/password login flow using `argon2` password hashing.
  - Persistent session-based auth powered by `tower-sessions` with a SQLx Postgres store — no in-memory shortcuts.

- **Postgres Integration**
  - Structured database layer built with `sqlx`.
  - Automated migrations for accounts, projects, and session tables.
  - Clean timestamp handling and UUIDs for resource tracking.

- **Rate Limiting**
  - Protects endpoints against abuse with configurable request limits.

- **Error Handling**
  - Custom error types using `thiserror` for consistent, readable API responses.

- **Integration Testing**
  - Tests simulate real API interactions with isolated environments.
  - Ensures confidence in authentication, CRUD functionality, and database integrity.

- **Docker-First Deployment**
  - Complete `Dockerfile` and `docker-compose.yml` setup.
  - Local environment spins up the API and Postgres database with a single command.

- **Modern Rust Stack**
  - Built on async Rust with `tokio`, `axum`, and `sqlx`.
  - Strong typing, safe concurrency, and security best practices.

---

## Quick Start (Docker)

\`\`\`bash
# Clone the repo
git clone https://github.com/yourname/axum-web.git
cd axum-web

# Build and run with Docker Compose
docker-compose up --build
\`\`\`

The API will be available at [http://localhost:3000](http://localhost:3000).

---

## Database Migrations

SQL migrations are located in the \`migrations/\` folder:

- \`create_accounts.sql\`: User table with hashed password storage.
- \`create_project_table.sql\`: Project resources linked to user accounts.
- \`create_sessions_table.sql\`: Session storage for persistent login state.

---

## Tech Stack

| Tool/Crate                | Purpose                                 |
|---------------------------|------------------------------------------|
| **Axum**                  | Web framework with router-based API     |
| **SQLx**                  | Async Postgres database interaction     |
| **Tower-Sessions**        | Production-grade session management     |
| **Argon2**                | Password hashing for authentication     |
| **Tokio**                 | Async runtime for high-performance I/O  |
| **Docker & Compose**      | Containerized development environment   |
| **Validator**             | Request payload validation              |
| **Rate Limiting**         | API protection from excessive requests  |
| **Integration Tests**     | End-to-end API validation               |

---

## Project Goals

This project serves as a portfolio centerpiece, showcasing:

- Realistic backend architecture
- Secure authentication implementation
- Database-driven state management
- Docker-based deployment workflows
- Idiomatic, maintainable Rust

---

## Future Improvements

Planned additions:

- Comprehensive API documentation (e.g., OpenAPI)
- Frontend example consuming the API
- CI pipeline for automated tests and builds

---

## Local Development

For non-Docker setups:

\`\`\`bash
# Ensure Postgres is running on port 5433 as defined in docker-compose.yml
export DATABASE_URL=postgres://postgres:postgres@localhost:5433/axum_db
cargo run
\`\`\`
