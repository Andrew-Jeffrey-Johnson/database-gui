
mod application_facilitators;
mod descriptions;

use tokio;

async fn setup() -> sqlx::Pool<sqlx::Postgres> {
    // Login information to database
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    // Connect to database
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    return pool;
}

pub fn create_description(original_text: &str) {
    let rt = tokio::runtime::Runtime::new();
    let _: Result<(), std::io::Error> = Ok(rt.expect("REASON").block_on(async {
        let pool = setup().await;
        let _ = descriptions::create_description(&pool, original_text).await;
    }));
}
