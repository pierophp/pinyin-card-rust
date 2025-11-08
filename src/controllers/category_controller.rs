use crate::models::category::Category;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct CategoryQuery {
    id: Option<i32>, // Optional parameter
}

/// Handler to get all categories from the database
pub async fn get_categories(
    State(pool): State<MySqlPool>,
    Query(params): Query<CategoryQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let where_clause = match params.id {
        Some(id) => format!("parent_category_id = {}", id),
        None => "parent_category_id IS NULL".to_string(),
    };

    let query = format!("SELECT * FROM category WHERE {}", where_clause);

    let categories = sqlx::query_as::<_, Category>(&query)
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
