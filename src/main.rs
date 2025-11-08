mod controllers;
mod models;
mod routes;

use sqlx::mysql::MySqlPoolOptions;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::from_filename(".env").ok();

    // Get database URL from environment
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    // Create database connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    println!("✅ Database connection established");

    // Build our application with routes
    let app = routes::configure_routes(pool);

    // Define the address to run the server on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("🚀 Server starting on http://{}", addr);

    // Run the server
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
