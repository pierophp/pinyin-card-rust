use crate::models::card::Card;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct CardQuery {
    category_id: i32, // Required parameter
}

/// Handler to get all cards from a category
pub async fn get_cards(
    State(pool): State<MySqlPool>,
    Query(params): Query<CardQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let query = "SELECT * FROM card WHERE category_id = ?";

    let cards = sqlx::query_as::<_, Card>(query)
        .bind(params.category_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch cards: {}", e),
            )
        })?;

    Ok(Json(cards))
}
