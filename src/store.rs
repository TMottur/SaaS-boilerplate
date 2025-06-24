use sqlx::{
    postgres::{PgPool, PgPoolOptions}
};
use serde::{Serialize, Deserialize};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Error
    },
    Argon2
};
use thiserror::Error;
use sqlx::Row;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub id: Option<uuid::Uuid>,
    pub user_email: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub last_updated: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub description: Option<String>,
    pub update_timestamp: chrono::NaiveDateTime
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
            {
                Ok(pool) => pool,
                Err(e) => panic!("Couldn't establish database connection: {}", e),
            };
        Store {
            connection: db_pool,
        }
    }

    pub async fn authenticate_user(&self, username: &str, password: &str) -> Result<bool, StoreError> {
        let row = sqlx::query("SELECT password FROM accounts WHERE username = $1")
            .bind(username)
            .fetch_one(&self.connection)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => StoreError::UserNotFound,
                other => StoreError::SqlxError(other),
            })?;

        let hashed = match row.try_get::<String, _>("password") {
            Ok(pwd) => pwd,
            Err(_) => return Err(StoreError::UserDataNotFound("password".to_string()))
        };

        let parsed_hash = match PasswordHash::new(&hashed) {
            Ok(ph) => ph,
            Err(_) => return Err(StoreError::MalformedStoreHash),
        };

        if Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok() {
                Ok(true)
            } else {
                Err(StoreError::IncorrectPassword)
            }
    }

    pub async fn register_user (&self, username: &str, password: &str) -> Result<(), StoreError> {
        let password = password.as_bytes();
        let hashed_password = Self::hash_password(password)
            .map_err(|e| StoreError::HashError(e.to_string()))?;
        sqlx::query(
            "INSERT INTO accounts (username, password) VALUES ($1, $2)",
        )
        .bind(username)
        .bind(&hashed_password)
        .execute(&self.connection)
        .await?;

    Ok(())
    }

    fn hash_password(password: &[u8]) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default().hash_password(password, &salt)?.to_string();
        Ok(hashed_password)
    }

    pub async fn create_project(&self, project: Project) -> Result<(), StoreError> {
        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO projects (id, user_email, name, description)
            VALUES ($1, $2, $3, $4)"
        )
        .bind(id)
        .bind(project.user_email)
        .bind(project.name)
        .bind(project.description)
        .execute(&self.connection)
        .await?;

    Ok(())
    }

    pub async fn list_projects(&self) -> Result<Vec<Project>, StoreError> {
        let projects = sqlx::query_as!(
            Project,
            "SELECT id, user_email, name, description, created_at, last_updated FROM projects"
        )
        .fetch_all(&self.connection)
        .await?;

        Ok(projects)
    }

    pub async fn get_project_by_id(
        &self, id: uuid::Uuid
    ) -> Result<Project, StoreError> {
        let project = sqlx::query_as!(
            Project,
            r#"
            SELECT id, user_email, name, description, created_at, last_updated
            FROM projects
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.connection)
        .await?;

        Ok(project)
    }
    
    pub async fn update_project (&self, update: UpdateProject, id: uuid::Uuid)
    -> Result<Project, StoreError> {
        let project = sqlx::query_as!(
            Project,
            r#"UPDATE projects SET name = $1, description = $2, last_updated = $3 WHERE id = $4
            RETURNING id, user_email, name, description, created_at, last_updated"#,
            update.name, update.description, update.update_timestamp, id
        )
        .fetch_one(&self.connection)
        .await?;

        Ok(project)
    }

    pub async fn delete_project(&self, id: uuid::Uuid)
    -> Result<(), StoreError> {
        let result = sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(id)
            .execute(&self.connection)
            .await?;

        if result.rows_affected() == 0 {
            return Err(StoreError::ProjectNotFound);
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("Password hashing failed: {0}")]
    HashError(String),

    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("User not found")]
    UserNotFound,

    #[error("Project not found")]
    ProjectNotFound,

    #[error("User data not found: {0}")]
    UserDataNotFound(String),

    #[error("Malformed store hash")]
    MalformedStoreHash,

    #[error("Incorrect Password")]
    IncorrectPassword,

    #[error("Failed to create project")]
    FailedProjectCreation,

}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for StoreError {
    fn into_response(self) -> Response {
        let status = match self {
            StoreError::UserNotFound | StoreError::IncorrectPassword => StatusCode::UNAUTHORIZED,
            StoreError::UserDataNotFound(_) | StoreError::ProjectNotFound | StoreError::MalformedStoreHash | StoreError::FailedProjectCreation => StatusCode::BAD_REQUEST,
            StoreError::HashError(_) | StoreError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(ErrorResponse {
            error: self.to_string(),
        });

        (status, body).into_response()
    }
}