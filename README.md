# SaaS Boilerplate (Rust + Axum)

This is a backend starter project built with [Axum](https://docs.rs/axum), designed as a boilerplate for SaaS-style applications. It includes user account management, project tracking, and structured error handling.

## Features

- **User Accounts**: Register users with email + hashed password
- **Projects API**: Full CRUD for user-owned project records
- **PostgreSQL + SQLx**: Async database access with typed queries
- **Modular Code Structure**: Split across `main.rs` and `store.rs`
- **Extensible Error Handling**: Custom error enum for API and DB errors
- **Environment-Safe**: `.env` support via `dotenvy` and `.gitignore`d for GitHub safety

## Current Stack

- **Rust**
- **Axum** (web framework)
- **SQLx** (Postgres driver)
- **dotenvy** (for config)
- **tokio** (async runtime)

## Getting Started

1. Set up your local Postgres database
2. Run migrations from `/migrations`
3. Create a `.env` file:
   ```env
   DATABASE_URL=postgres://<user>:<password>@localhost:5432/<your_db>
   ```
4. Build and run:
   ```bash
   cargo run
   ```

## Folder Structure

```bash
src/
├── main.rs        # Axum router and app setup
├── store.rs       # Database layer
migrations/
├── create_accounts.sql
├── create_project_table.sql
.env.example        # Example config (safe to commit)
.gitignore          # Ignores .env and other local files
```

## 🛠 Todo

- ✅ Basic account and project management
- 🔒 Add authentication middleware
- 🌐 Add frontend (Yew or Leptos?)
- 📦 Dockerize for deployment

---

## 🧠 Notes

This is a portfolio project used for Rust backend practice and experimentation. It is not production-ready but structured cleanly for future expansion and deployment.
