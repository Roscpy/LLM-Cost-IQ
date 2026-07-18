// tests/analyze_integration.rs
use serde_json::json;

#[tokio::test]
async fn full_analyze_flow() {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&database_url).await.unwrap();
    
    // 1. Login pour obtenir token
    let client = reqwest::Client::new();
    let login_resp = client.post("http://localhost:3000/login")
        .json(&json!({"email": "test_integration@example.com"}))
        .send().await.unwrap();
    let token = login_resp.json::<serde_json::Value>().await.unwrap()["token"].as_str().unwrap().to_string();

    // 2. POST /analyze
    let analyze_resp = client.post("http://localhost:3000/analyze")
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({"prompt": "Hello world, this is a test prompt for LLM cost estimation"}))
        .send().await.unwrap();
    assert_eq!(analyze_resp.status(), 200);
    let analysis = analyze_resp.json::<serde_json::Value>().await.unwrap();
    assert!(analysis["tokens"].as_i64().unwrap() > 0);
    assert!(analysis["cost"].as_f64().unwrap() > 0.0);
    assert!(analysis["ai_cost_score"].as_i64().unwrap() >= 0);
    assert!(analysis["ai_cost_score"].as_i64().unwrap() <= 100);

    // 3. GET /history
    let history_resp = client.get("http://localhost:3000/history")
        .header("Authorization", format!("Bearer {}", token))
        .send().await.unwrap();
    assert_eq!(history_resp.status(), 200);
    let history = history_resp.json::<serde_json::Value>().await.unwrap();
    assert!(history.as_array().unwrap().len() >= 1);

    // 4. GET /dashboard/stats
    let stats_resp = client.get("http://localhost:3000/dashboard/stats")
        .header("Authorization", format!("Bearer {}", token))
        .send().await.unwrap();
    assert_eq!(stats_resp.status(), 200);
    let stats = stats_resp.json::<serde_json::Value>().await.unwrap();
    assert!(stats["total_analyses"].as_i64().unwrap() >= 1);
    assert!(stats["total_cost"].as_f64().unwrap() >= 0.0);
}
