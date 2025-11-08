use crate::controllers::hello_controller;
use axum::{Router, routing::get};

/// Configure all application routes
pub fn configure_routes() -> Router {
    Router::new()
        .route("/", get(hello_controller::hello_world))
        .route("/health", get(hello_controller::health_check))
}
