
use tokio;
use chrono::offset::TimeZone;
use chrono::offset::Utc;

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
        let _ = create_description_async(&pool, original_text).await;
    }));
}

pub fn get_experience(id: i32) -> Result<ExperienceSql, sqlx::Error>{
    let rt = tokio::runtime::Runtime::new();
    let experience = rt.expect("REASON").block_on(async {
        let pool = setup().await;
        get_experience_sql(&pool, id).await
    });
    return experience;
}

pub fn get_all_experiences() -> Result<Vec<ExperienceSql>, sqlx::Error>{
    let rt = tokio::runtime::Runtime::new();
    let experiences = rt.expect("REASON").block_on(async {
        let pool = setup().await;
        let experience_sql = 
            sqlx::query_as::<_, ExperienceSql>("SELECT * FROM \"Experiences\"")
            .fetch_all(&pool)
            .await?;
        Ok(experience_sql)
    });
    return experiences;
}


// ------------------ DESCRIPTION ---------------------------------------------
#[derive(sqlx::FromRow)]
pub struct Description {
    pub id: i32,
    pub original_text: String,
}

pub async fn create_description_async(
    pool: &sqlx::PgPool, 
    original_text: &str) 
    -> Result<(), sqlx::Error> 
{
    sqlx::query("INSERT INTO public.\"Descriptions\" (original_text) VALUES ($1)")
        .bind(original_text)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_description(
    pool: &sqlx::PgPool, 
    description_id: i32) 
    -> Result<Description, sqlx::Error> 
{
    let description = sqlx::query_as::<_, Description>("SELECT * FROM public.\"Descriptions\" WHERE id = $1")
        .bind(description_id)
        .fetch_one(pool)
        .await?;
    Ok(description)
}

pub async fn update_description(
    pool: &sqlx::PgPool, 
    description_id: i32, 
    original_text: &str) 
    -> Result<(), sqlx::Error> 
{
    sqlx::query("UPDATE public.\"Descriptions\" SET original_text = $1 WHERE id = $2")
        .bind(original_text)
        .bind(description_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_description(
    pool: &sqlx::PgPool, 
    description_id: i32) 
    -> Result<(), sqlx::Error> 
{
    sqlx::query("DELETE FROM public.\"Descriptions\" WHERE id = $1")
        .bind(description_id)
        .execute(pool)
        .await?;
    Ok(())
}

// ------------------ ApplicationFacilitator ----------------------------------
#[derive(sqlx::FromRow)]
pub struct ApplicationFacilitator {
    pub id: i32,
    pub name: String,
    pub website: String,
}

pub async fn create_application_facilitator(
    pool: &sqlx::PgPool, 
    name: &str, 
    website: &str) 
    -> Result<(), sqlx::Error> 
{
    sqlx::query("INSERT INTO ApplicationFacilitators (name, website) VALUES ($1, $2)")
        .bind(name)
        .bind(website)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_application_facilitator(
    pool: &sqlx::PgPool, 
    application_facilitator_id: i32) 
    -> Result<ApplicationFacilitator, sqlx::Error> 
{
    let application_facilitator = 
        sqlx::query_as::<_, ApplicationFacilitator>("SELECT * FROM ApplicationFacilitators WHERE id = $1")
        .bind(application_facilitator_id)
        .fetch_one(pool)
        .await?;
    Ok(application_facilitator)
}

pub async fn update_application_facilitator(
    pool: &sqlx::PgPool, 
    application_facilitator_id: i32, 
    new_name: &str,
    new_website: &str) 
    -> Result<(), sqlx::Error> 
{
    sqlx::query("UPDATE ApplicationFacilitators SET email = $1, application_facilitator_id = $2 WHERE id = $3")
        .bind(new_name)
        .bind(new_website)
        .bind(application_facilitator_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_application_facilitator(
    pool: &sqlx::PgPool, 
    application_facilitator_id: i32) 
    -> Result<(), sqlx::Error> 
{
    sqlx::query("DELETE FROM ApplicationFacilitators WHERE id = $1")
        .bind(application_facilitator_id)
        .execute(pool)
        .await?;
    Ok(())
}

// ------------------ Experience ----------------------------------
#[derive(sqlx::FromRow)]
pub struct ExperienceSql {
    pub id: i32,
    pub employing_entity_id: i32,
    pub title: String,
}

pub async fn create_experience_sql(
    pool: &sqlx::PgPool, 
    employing_entity_id: i32, 
    title: &str) 
    -> Result<(), sqlx::Error> 
{
    sqlx::query("INSERT INTO \"Experiences\" (employing_entity_id, title) VALUES ($1, $2)")
        .bind(employing_entity_id)
        .bind(title)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_experience_sql(
    pool: &sqlx::PgPool, 
    experience_sql_id: i32) 
    -> Result<ExperienceSql, sqlx::Error> 
{
    let experience_sql = 
        sqlx::query_as::<_, ExperienceSql>("SELECT * FROM \"Experiences\" WHERE id = $1")
        .bind(experience_sql_id)
        .fetch_one(pool)
        .await?;
    Ok(experience_sql)
}

pub async fn get_all_experience_sql(pool: &sqlx::PgPool) -> Result<Vec<ExperienceSql>, sqlx::Error> 
{
    let experience_sql = 
        sqlx::query_as::<_, ExperienceSql>("SELECT * FROM \"Experiences\"")
        .fetch_all(pool)
        .await?;
    Ok(experience_sql)
}
