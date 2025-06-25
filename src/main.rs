use axum::{
    routing::{post, get, put, delete},
    Router,
    http::StatusCode,
    response::IntoResponse,
    extract::{Json, State, Path},
};
use serde::Deserialize;
use tower_sessions::{Expiry, SessionManagerLayer, Session, cookie::time::Duration, MemoryStore};
use std::{sync::Arc};
use sqlx::migrate;

mod store;
mod auth;

#[tokio::main]
async fn main() {
    dotenvy::from_filename(".env").ok();
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let user_password_db = store::Store::new(&db_url).await;

    migrate!()
        .run(&user_password_db.connection)
        .await
        .expect("Failed to run migrations");

    let shared_store = Arc::new(user_password_db);

    let memory_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(memory_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(10)));

    let app = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/signup", post(add_user))
        .route("/projects", post(create_project_handler))
        .route("/projects", get(list_project_handler))
        .route("/projects/:id", get(get_project_by_id_handler))
        .route("/projects/:id", put(update_project_handler))
        .route("/projects/:id", delete(delete_project_handler))
        .route("/healthz", get(|| async { "Server Check: OK" }))
        .with_state(shared_store)
        .layer(session_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn login(
    State(store): State<Arc<store::Store>>,
    session: Session,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, store::StoreError> {
    store.authenticate_user(&payload.username, &payload.password).await?;

    session.insert("user_email", &payload.username).await
        .map_err(|_| store::StoreError::SessionError)?;
    Ok(StatusCode::OK)
}

async fn logout(
    session: Session,
) -> Result<impl IntoResponse, store::StoreError> {
    session.clear().await;

    Ok(StatusCode::OK)
}

async fn add_user(
    State(store): State<Arc<store::Store>>,
    Json(payload): Json<NewUser>,
) -> Result<impl IntoResponse, store::StoreError> {
    match store.register_user(&payload.username, &payload.password).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            eprintln!("Signup error: {:?}", e);
            Err(e)
        }
    }
}

async fn create_project_handler(
    State(store): State<Arc<store::Store>>,
    session: Session,
    Json(project): Json<store::Project>,
) -> Result<StatusCode, store::StoreError> {
    require_login(&session).await?;
    store.create_project(project)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| store::StoreError::FailedProjectCreation)
}

async fn list_project_handler(
    State(store): State<Arc<store::Store>>,
    session: Session,
) -> Result<StatusCode, store::StoreError> {
    require_login(&session).await?;
    store.list_projects()
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| store::StoreError::ProjectNotFound)
}

async fn get_project_by_id_handler(
    State(store): State<Arc<store::Store>>,
    session: Session,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, store::StoreError> {
    require_login(&session).await?;
    store.get_project_by_id(id)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| store::StoreError::ProjectNotFound)
}

async fn update_project_handler(
    State(store): State<Arc<store::Store>>,
    session: Session,
    Path(id): Path<uuid::Uuid>,
    Json(update): Json<store::UpdateProject>
) -> Result<Json<store::Project>, store::StoreError> {
    require_login(&session).await?;
    store.update_project(update, id)
        .await
        .map(Json)
}

async fn delete_project_handler(
    State(store): State<Arc<store::Store>>,
    session: Session,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, store::StoreError> {
    require_login(&session).await?;
    store.delete_project(id)
        .await
        .map(|_| StatusCode::OK)
}

async fn require_login(session: &Session) -> Result<String, store::StoreError> {
    session.get::<String>("user_email").await
        .map_err(|_| store::StoreError::SessionError)?
        .ok_or(store::StoreError::IncorrectPassword)
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct NewUser {
    username: String,
    password: String,
}


