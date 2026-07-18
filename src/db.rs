// src/db.rs (fichier séparé)
use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;

pub async fn connect_db(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}
