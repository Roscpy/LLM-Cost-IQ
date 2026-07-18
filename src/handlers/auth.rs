// src/handlers/auth.rs
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::auth::create_token;

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    // mot de passe volontairement ignoré pour MVP simplifié
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, axum::http::StatusCode> {
    // Pour MVP: on crée l'utilisateur s'il n'existe pas, on renvoie un token
    let user_id = sqlx::query_scalar::<_, uuid::Uuid>(
        "INSERT INTO users (email) VALUES ($1) ON CONFLICT (email) DO NOTHING RETURNING id"
    )
    .bind(&req.email)
    .fetch_optional(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let id = match user_id {
        Some(id) => id.to_string(),
        None => {
            // récupérer l'id existant
            sqlx::query_scalar::<_, uuid::Uuid>("SELECT id FROM users WHERE email = $1")
                .bind(&req.email)
                .fetch_one(&pool)
                .await
                .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?
                .to_string()
        }
    };
    let token = create_token(&id).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(LoginResponse { token }))
      }
