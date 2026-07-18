// tests/db_test.rs (intégration)
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

#[tokio::test]
async fn test_db_connection() {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://costiq:costiq123@localhost:5432/costiq".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .unwrap();
    let row: (i64,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, 1);
}
