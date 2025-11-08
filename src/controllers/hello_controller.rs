use crate::models::response::HelloResponse;
use axum::{Json, response::IntoResponse};

/// Handler for the root endpoint
pub async fn hello_world() -> impl IntoResponse {
    Json(HelloResponse {
        message: "Hello, Piero!".to_string(),
    })
}

/// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
