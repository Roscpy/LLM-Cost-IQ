// src/main.rs
mod db;
mod auth;
mod handlers;

use axum::{Router, routing::post, extract::State};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://costiq:costiq123@localhost:5432/costiq".to_string());
    let pool = PgPool::connect(&database_url).await?;

    let app = Router::new()
        .route("/health", get(health))
        .route("/login", post(handlers::auth::login))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
use axum::{Router, routing::get};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/health", get(health));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn health() -> &'static str {
    "OK"
}


