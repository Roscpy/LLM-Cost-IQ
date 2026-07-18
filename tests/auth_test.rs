// tests/auth_test.rs
use axum::Router;
use axum::routing::post;
use axum::extract::State;
use sqlx::PgPool;
use serde_json::json;

#[tokio::test]
async fn test_login_creates_user_and_returns_token() {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://costiq:costiq123@localhost:5432/costiq".to_string());
    let pool = PgPool::connect(&database_url).await.unwrap();
    let app = Router::new()
        .route("/login", post(handlers::auth::login))
        .with_state(pool.clone());
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });
    let client = reqwest::Client::new();
    let resp = client.post(&format!("http://{}/login", addr))
        .json(&json!({"email": "test@example.com"}))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("token").is_some());
}
