use axum::{
    routing::{post, get, put, delete},
    Router,
    http::StatusCode,
    response::IntoResponse,
    extract::{Json, State, Path},
};
use serde::Deserialize;
use tower_sessions::{Expiry, SessionManagerLayer, Session, cookie::time::Duration};
use std::{sync::Arc};
use sqlx::migrate;
use tower_sessions_sqlx_store::PostgresStore;
use validator::Validate;

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
    
    let memory_store = PostgresStore::new(shared_store.connection.clone());
    let session_layer = SessionManagerLayer::new(memory_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(10)));
     

    let app = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/signup", post(add_user))
        .route("/projects", post(create_project_handler))
        .route("/projects", get(list_project_handler))
        .route("/projects/{id}", get(get_project_by_id_handler))
        .route("/projects/{id}", put(update_project_handler))
        .route("/projects/{id}", delete(delete_project_handler))
        .route("/healthz", get(health_check))
        .with_state(shared_store)
        .layer(session_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn login(
    State(store): State<Arc<store::Store>>,
    session: Session,
    _rate_limit: auth::RateLimit,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, store::StoreError> {
    store.authenticate_user(&payload.email, &payload.password).await?;

    session.insert("user_email", &payload.email).await
        .map_err(|_| store::StoreError::SessionError)?;
    Ok(StatusCode::OK)
}

async fn logout(
    session: Session,
    _rate_limit: auth::RateLimit,
) -> Result<impl IntoResponse, store::StoreError> {
    session.clear().await;

    Ok(StatusCode::OK)
}

async fn add_user(
    State(store): State<Arc<store::Store>>,
    Json(payload): Json<NewUser>,
) -> Result<impl IntoResponse, store::StoreError> {
    // Validate the payload user data
    if let Err(e) = payload.validate() {
        return Err(store::StoreError::InvalidInput(format!("{}", e)));
    }
    match store.register_user(&payload.email, &payload.password).await {
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
    _rate_limit: auth::RateLimit,
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
    _rate_limit: auth::RateLimit,
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
    _rate_limit: auth::RateLimit,
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
    _rate_limit: auth::RateLimit,
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
    _rate_limit: auth::RateLimit,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, store::StoreError> {
    require_login(&session).await?;
    store.delete_project(id)
        .await
        .map(|_| StatusCode::OK)
}

async fn health_check(State(store): State<Arc<store::Store>>) -> impl IntoResponse {
    if let Err(e) = sqlx::query("SELECT 1").execute(&store.connection).await {
        eprintln!("Health check DB error: {:?}", e);
        return StatusCode::SERVICE_UNAVAILABLE;
    }
    StatusCode::OK
}


async fn require_login(session: &Session) -> Result<String, store::StoreError> {
    session.get::<String>("user_email").await
        .map_err(|_| store::StoreError::SessionError)?
        .ok_or(store::StoreError::IncorrectPassword)
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize, Validate)]
struct NewUser {
    #[validate(email)]
    email: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    password: String,
}


