// src/models/analysis.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Analysis {
    pub id: Uuid,
    pub user_id: Uuid,
    pub prompt: String,
    pub tokens: i32,
    pub cost: f64,
    pub ai_cost_score: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AnalyzeRequest {
    pub prompt: String,
}

#[derive(Debug, Serialize)]
pub struct AnalyzeResponse {
    pub id: Uuid,
    pub tokens: i32,
    pub cost: f64,
    pub ai_cost_score: i32,
    pub prompt: String,
    pub created_at: DateTime<Utc>,
  }
