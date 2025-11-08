use crate::models::response::HelloResponse;
use axum::{Json, response::IntoResponse};

/// Handler for the root endpoint
pub async fn hello_world() -> impl IntoResponse {
    Json(HelloResponse {
        message: "Hello, World!".to_string(),
    })
}

/// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Local::now().to_rfc3339(),
    }))
}
