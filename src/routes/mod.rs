use crate::controllers::{card_controller, category_controller, hello_controller};
use axum::{Router, http::HeaderValue, routing::get};
use sqlx::MySqlPool;
use tower_http::cors::{Any, CorsLayer};

pub fn configure_routes(pool: MySqlPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://localhost:3001"))
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", get(hello_controller::hello_world))
        .route("/health", get(hello_controller::health_check))
        .route("/categories", get(category_controller::get_categories))
        .route("/cards", get(card_controller::get_cards))
        .layer(cors)
        .with_state(pool)
}
