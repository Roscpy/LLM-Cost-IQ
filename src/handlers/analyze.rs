// src/handlers/analyze.rs
use axum::{Json, extract::State, Extension};
use sqlx::PgPool;
use crate::models::analysis::{Analysis, AnalyzeRequest, AnalyzeResponse};
use crate::services::estimator;
use uuid::Uuid;

pub async fn analyze_prompt(
    State(pool): State<PgPool>,
    Extension(user_id): Extension<Uuid>,
    Json(req): Json<AnalyzeRequest>,
) -> Result<Json<AnalyzeResponse>, axum::http::StatusCode> {
    let tokens = estimator::estimate_tokens(&req.prompt);
    let cost = estimator::estimate_cost(tokens);
    let score = estimator::calculate_ai_cost_score(tokens, cost);

    let analysis = sqlx::query_as::<_, Analysis>(
        "INSERT INTO analyses (user_id, prompt, tokens, cost, ai_cost_score)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, user_id, prompt, tokens, cost, ai_cost_score, created_at"
    )
    .bind(user_id)
    .bind(&req.prompt)
    .bind(tokens)
    .bind(cost)
    .bind(score)
    .fetch_one(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AnalyzeResponse {
        id: analysis.id,
        tokens: analysis.tokens,
        cost: analysis.cost,
        ai_cost_score: analysis.ai_cost_score,
        prompt: analysis.prompt,
        created_at: analysis.created_at,
    }))
      }
