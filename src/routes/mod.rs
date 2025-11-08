use crate::controllers::{category_controller, hello_controller};
use axum::{Router, routing::get};
use sqlx::MySqlPool;

/// Configure all application routes
pub fn configure_routes(pool: MySqlPool) -> Router {
    Router::new()
        .route("/", get(hello_controller::hello_world))
        .route("/health", get(hello_controller::health_check))
        .route("/categories", get(category_controller::get_categories))
        .with_state(pool)
}
