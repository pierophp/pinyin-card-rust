use crate::models::category::Category;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::MySqlPool;

/// Handler to get all categories from the database
pub async fn get_categories(
    State(pool): State<MySqlPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let categories = sqlx::query_as::<_, Category>("SELECT * FROM category")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch categories: {}", e),
            )
        })?;

    Ok(Json(categories))
}

