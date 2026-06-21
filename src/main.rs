use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub description: String,
    pub url: Option<String>,
}

impl Project {
    fn new(title: String, description: String, url: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            url,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    title: String,
    description: String,
    url: Option<String>,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Project not found")]
    NotFound,
    #[error("Invalid input")]
    ValidationError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(serde_json::json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

pub struct AppState {
    projects: RwLock<Vec<Project>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState {
        projects: RwLock::new(Vec::new()),
    });

    let app = Router::new()
        .route("/projects", get(list_projects).post(create_project))
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port 3000");

    info!("Server listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

async fn health_check() -> &'static str {
    "OK"
}

async fn list_projects(State(state): State<Arc<AppState>>) -> Json<Vec<Project>> {
    let projects = state.projects.read().await;
    Json(projects.clone())
}

async fn create_project(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateProjectRequest>,
) -> (StatusCode, Json<Project>) {
    let project = Project::new(payload.title, payload.description, payload.url);
    state.projects.write().await.push(project.clone());

    info!("Created project: {}", project.id);
    (StatusCode::CREATED, Json(project))
}
