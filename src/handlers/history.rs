// src/handlers/history.rs
use axum::{Json, extract::State, Extension};
use sqlx::PgPool;
use crate::models::analysis::Analysis;
use uuid::Uuid;

pub async fn get_history(
    State(pool): State<PgPool>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<Vec<Analysis>>, axum::http::StatusCode> {
    let analyses = sqlx::query_as::<_, Analysis>(
        "SELECT id, user_id, prompt, tokens, cost, ai_cost_score, created_at
         FROM analyses
         WHERE user_id = $1
         ORDER BY created_at DESC
         LIMIT 100"
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(analyses))
}
