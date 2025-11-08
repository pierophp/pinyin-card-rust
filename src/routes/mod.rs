use crate::controllers::{category_controller, hello_controller};
use axum::{Router, http::HeaderValue, routing::get};
use sqlx::MySqlPool;
use tower_http::cors::{Any, CorsLayer};

/// Configure all application routes
pub fn configure_routes(pool: MySqlPool) -> Router {
    // Configure CORS to allow requests from localhost:5173
    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://localhost:5173"))
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", get(hello_controller::hello_world))
        .route("/health", get(hello_controller::health_check))
        .route("/categories", get(category_controller::get_categories))
        .layer(cors)
        .with_state(pool)
}
