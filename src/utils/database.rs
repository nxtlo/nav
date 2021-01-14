use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;



pub async fn pool_init() -> Result<PgPool, Box<dyn std::error::Error + Send + Sync>> {

    let dsn = env::var("DSN_URL")?;

    let pool = PgPoolOptions::new()
        .max_connection(10)
        .connect(&dsn)
        .await?;

    Ok(pool)
}