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
pub fn fetch(expr: &String) -> Vec<sqlx::postgres::PgRow> {
    let rt = tokio::runtime::Runtime::new();
    let all_rows = rt.expect("Couldn't create tokio block").block_on(async {
        let pool = setup().await;
        sqlx::query(expr)
            .fetch_all(&pool)
            .await
            .expect(format!("SQL query failed: {expr}").as_str())
    });
    return all_rows;
}
pub fn fetch_one(expr: &String) -> sqlx::postgres::PgRow {
    let rt = tokio::runtime::Runtime::new();
    let row = rt.expect("Couldn't create tokio block").block_on(async {
        let pool = setup().await;
        sqlx::query(expr)
            .fetch_one(&pool)
            .await
            .expect(format!("SQL query failed: {expr}").as_str())
    });
    return row;
}
