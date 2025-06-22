use axum::{
    routing::{post, get, put, delete},
    Router,
    http::StatusCode,
    response::IntoResponse,
    extract::{Json, State, Path},
};
use serde::Deserialize;
use std::sync::Arc;
use sqlx::migrate;

mod store;

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

    let app = Router::new()
        .route("/login", post(login))
        .route("/signup", post(add_user))
        .route("/projects", post(create_project_handler))
        .route("/projects", get(list_project_handler))
        .route("/projects/:id", get(get_project_by_id_handler))
        .route("/projects/:id", put(update_project_handler))
        .route("/projects/:id", delete(delete_project_handler))
        .with_state(shared_store);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn login(
    State(store): State<Arc<store::Store>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, store::StoreError> {
    store.authenticate_user(&payload.username, &payload.password).await?;
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
    Json(project): Json<store::Project>,
) -> Result<StatusCode, StatusCode> {
    store.create_project(project)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn list_project_handler(
    State(store): State<Arc<store::Store>>,
) -> Result<StatusCode, StatusCode> {
    store.list_projects()
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_project_by_id_handler(
    State(store): State<Arc<store::Store>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, StatusCode> {
    store.get_project_by_id(id)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn update_project_handler(
    State(store): State<Arc<store::Store>>,
    Path(id): Path<uuid::Uuid>,
    Json(update): Json<store::UpdateProject>
) -> Result<Json<store::Project>, store::StoreError> {
    store.update_project(update, id)
        .await
        .map(Json)
}

async fn delete_project_handler(
    State(store): State<Arc<store::Store>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, store::StoreError> {
    store.delete_project(id)
        .await
        .map(|_| StatusCode::OK)
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


