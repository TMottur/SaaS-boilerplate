
# SaaS Boilerplate API - Axum, Rust, Postgres

This project is a scalable SaaS boilerplate backend built with **Rust** and **Axum**, designed as a secure, minimal foundation for web applications. It provides user authentication, session management, and full CRUD operations for project resources, backed by a Postgres database.

---

## Features

**Axum Framework** — Fast, ergonomic Rust web server  
**PostgreSQL Integration** — Data persistence via SQLx with Postgres  
**Secure Password Hashing** — User credentials protected with `argon2`  
**Session Management** — Cookie-based sessions using `tower-sessions`  
**User Authentication** — Account creation and login routes implemented  
**Project Resource CRUD** — Create, Read, Update, and Delete projects  
**UUID Project IDs** — Unique project identifiers with `pgcrypto` extension  
**Timestamps** — Tracks creation and modification times for projects  
**Rust Error Handling** — Idiomatic, custom error types with `thiserror`  
**Environment Config** — Uses `dotenvy` for environment variable management  

---

## 🛠️ Tech Stack

- **Rust**  
- **Axum**  
- **SQLx (Postgres driver)**  
- **tower-sessions**  
- **Argon2 password hashing**  
- **Tokio async runtime**  
- **Postgres Database**  
- **dotenvy**  

---

## 📚 Current API Endpoints (Highlights)

| Method | Endpoint         | Description                           |
|--------|-----------------|---------------------------------------|
| `POST` | `/signup`       | Register new user account             |
| `POST` | `/login`        | Authenticate and create session       |
| `POST` | `/logout`       | Invalidate user session (logout)      |
| `POST` | `/projects`     | Create new project (auth required)    |
| `GET`  | `/projects`     | List all projects for user (auth)     |
| `PUT`  | `/projects/:id` | Update existing project (auth)        |
| `DELETE` | `/projects/:id` | Delete project by ID (auth)        |


