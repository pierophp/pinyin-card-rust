use axum::{
    routing::get,
    Router,
    Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct HelloResponse {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct ApiInfo {
    name: String,
    version: String,
    endpoints: Vec<String>,
}

// Handler for the root endpoint
async fn hello_world() -> impl IntoResponse {
    Json(HelloResponse {
        message: "Hello, World!".to_string(),
    })
}

// Handler for API info
async fn api_info() -> impl IntoResponse {
    Json(ApiInfo {
        name: "Pinyin Card Rust API".to_string(),
        version: "0.1.0".to_string(),
        endpoints: vec![
            "/".to_string(),
            "/api/info".to_string(),
            "/api/health".to_string(),
        ],
    })
}

// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[tokio::main]
async fn main() {
    // Build our application with routes
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/api/info", get(api_info))
        .route("/api/health", get(health_check));

    // Define the address to run the server on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    println!("🚀 Server starting on http://{}", addr);
    println!("📋 Available endpoints:");
    println!("   GET http://{}/", addr);
    println!("   GET http://{}/api/info", addr);
    println!("   GET http://{}/api/health", addr);

    // Run the server
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

