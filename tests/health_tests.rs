// tests/health_test.rs
use axum::Router;
use axum::routing::get;
use axum::response::IntoResponse;

#[tokio::test]
async fn test_health() {
    let app = Router::new().route("/health", get(|| async { "OK" }));
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
    let resp = client.get(&format!("http://{}/health", addr)).send().await.unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.text().await.unwrap(), "OK");
}
