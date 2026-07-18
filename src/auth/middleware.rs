// src/auth/middleware.rs
use axum::{middleware::Next, response::Response, http::{Request, StatusCode, header::AUTHORIZATION}, extract::State};
use crate::auth::verify_token;
use uuid::Uuid;

pub async fn auth_middleware<B>(
    State(pool): State<sqlx::PgPool>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = verify_token(auth_header).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Vérifier que l'utilisateur existe en base
    let exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)")
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if !exists {
        return Err(StatusCode::UNAUTHORIZED);
    }

    req.extensions_mut().insert(user_id);
    Ok(next.run(req).await)
}
