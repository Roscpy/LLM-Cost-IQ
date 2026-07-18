// src/handlers/dashboard.rs
use axum::{Json, extract::State, Extension};
use sqlx::PgPool;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct DashboardStats {
    pub total_analyses: i64,
    pub total_cost: f64,
    pub avg_tokens: f64,
    pub avg_score: f64,
    pub best_score: i32,
    pub worst_score: i32,
}

pub async fn get_dashboard_stats(
    State(pool): State<PgPool>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<DashboardStats>, axum::http::StatusCode> {
    let stats = sqlx::query_as::<_, (i64, f64, f64, f64, i32, i32)>(
        "SELECT 
            COUNT(*)::bigint,
            COALESCE(SUM(cost), 0.0),
            COALESCE(AVG(tokens), 0.0),
            COALESCE(AVG(ai_cost_score), 0.0),
            COALESCE(MAX(ai_cost_score), 0),
            COALESCE(MIN(ai_cost_score), 0)
         FROM analyses
         WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(DashboardStats {
        total_analyses: stats.0,
        total_cost: stats.1,
        avg_tokens: stats.2,
        avg_score: stats.3,
        best_score: stats.4,
        worst_score: stats.5,
    }))
}
