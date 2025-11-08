mod controllers;
mod models;
mod routes;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Build our application with routes
    let app = routes::configure_routes();

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
