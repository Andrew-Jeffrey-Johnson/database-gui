
use tokio;
use chrono::offset::TimeZone;
use chrono::offset::Utc;
use sqlx::Row;


#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct PostalAddressSQL {
   pub id: i32,
   pub name: String,
   pub line_1: String,
   pub line_2: String,
   pub line_3: String,
   pub city: String,
   pub state: String,
   pub zip_code: String,
}
struct ContactSQL {
   pub id: i32,
   pub first_name: String,
   pub last_name: String,
   pub username: String,
   pub email: String,
   pub phone_number: String,
   pub url: String,
   pub address_id: i32,
}
struct EmplyingEntitySQL {
   pub id: i32,
   pub name: String,
   pub url: String,
   pub headquarters_postal_address_id: i32,
}
struct ExperienceSQL {
   pub id: i32,
   pub employing_entity_id: i32,
   pub start_timestamptz: chrono::DateTime<chrono::Utc>,
   pub end_timestamptz: chrono::DateTime<chrono::Utc>,
   pub postal_address_id: i32,
}
struct AchievementSQL {
    id: i32,
    experience_id: i32,
    short_description: String,
    defense: String,
}
struct AchievementVariantSQL {
    id: i32,
    achievement_id: i32,
    description: String,
    defense: String,
}
struct ProjectSQL {
    id: i32,
    start_timestamptz: chrono::DateTime<chrono::Utc>,
    end_timestamptz: chrono::DateTime<chrono::Utc>,
    name: String,
    url: String,
    postal_address_id: i32,
}
struct ProjectHighlightSQL {
    id: i32,
    project_id: i32,
    short_description: String,
    defense: String,
}
struct ProjectHighlightVariantSQL {
    id: i32,
    project_highlight_id: i32,
    description: String,
    defense: String,
}
struct ListingHostSQL {
    id: i32,
    name: String,
    url: String,
}
struct ListingSQL {
    id: i32,
    listing_host_id: i32,
    description: String,
    posted_timestamptz: chrono::DateTime<chrono::Utc>,
    recruiter_contact_id: i32,
}
struct ApplicationSQL {
    id: i32,
    start_timestamptz: chrono::DateTime<chrono::Utc>,
    submitted_timestamptz: chrono::DateTime<chrono::Utc>,
    listing_id: i32,
}
struct ApplicationSelectionSQL {
    id: i32,
    application_id: i32,
    achievement_variant_id: i32,
}
struct ApplicationQuestionAnswerSQL {
    id: i32,
    application_id: i32,
    question: String,
    answer: String,
}

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
// *********************** NEW FUNCTIONS *********************************************
pub fn create_postal_address(
    name: &String, 
    line_1: &String, 
    line_2: &String,
    line_3: &String,
    city: &String,
    state: &String,
    zip_code: &String,
) -> i32 {
    let rt = tokio::runtime::Runtime::new();
    let id: i32 = rt.expect("REASON").block_on(async {
        let pool = setup().await;
        let id_raw = sqlx::query(
            "INSERT INTO 
            \"postal_address\" (name, line_1, line_2, line_3, city, state, zip_code) 
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id")
            .bind(name)
            .bind(line_1)
            .bind(line_2)
            .bind(line_3)
            .bind(city)
            .bind(state)
            .bind(zip_code)
            .fetch_one(&pool)
            .await;
        id_raw.unwrap().get::<i32, usize>(0)
    });
    return id;
}
pub fn fetch_all_from_postal_address() -> Vec<(i32, String, String, String, String, String, String, String)> {
    let rt = tokio::runtime::Runtime::new();
    let all_rows = rt.expect("REASON").block_on(async {
        let pool = setup().await;
        let raw: Vec<(i32, String, String, String, String, String, String, String)> = sqlx::query_as(
            "SELECT 
                id,
                name,
                line_1,
                line_2,
                line_3,
                city,
                state,
                zip_code
            FROM
                postal_address
            ORDER BY
                id
            ")
            .fetch_all(&pool)
            .await.expect("REASON");
        raw
    });
    return all_rows;
}
pub fn fetch_one_from_postal_address(id: i32) -> (i32, String, String, String, String, String, String, String) {
    let rt = tokio::runtime::Runtime::new();
    let row = rt.expect("REASON").block_on(async {
        let pool = setup().await;
        sqlx::query_as::<_, (i32, String, String, String, String, String, String, String)>(
            "SELECT 
                id,
                name,
                line_1,
                line_2,
                line_3,
                city,
                state,
                zip_code
            FROM
                postal_address
            WHERE
                id = $1
            ")
            .bind(id)
            .fetch_one(&pool)
            .await
    });
    return row.expect("Can't unwrap address row");
}
pub fn create_contact(
    first_name: &String, 
    last_name: &String, 
    username: &String,
    email: &String,
    phone_number: &String,
    url: &String,
    address_id: i32,
) -> i32 {
    let rt = tokio::runtime::Runtime::new();
    let id: i32 = rt.expect("REASON").block_on(async {
        let pool = setup().await;
        let id_raw = sqlx::query(
            "INSERT INTO 
            \"contact\" (first_name, last_name, username, email, phone_number, url, address_id) 
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id")
            .bind(first_name)
            .bind(last_name)
            .bind(username)
            .bind(email)
            .bind(phone_number)
            .bind(url)
            .bind(address_id)
            .fetch_one(&pool)
            .await;
        id_raw.unwrap().get::<i32, usize>(0)
    });
    return id;
}
pub fn create_employing_entity(
    name: &String, 
    url: &String,
    address_id: i32,
) -> i32 {
    let rt = tokio::runtime::Runtime::new();
    let id: i32 = rt.expect("REASON").block_on(async {
        let pool = setup().await;
        let id_raw = sqlx::query(
            "INSERT INTO 
            \"employing_entity\" (name, url, address_id) 
            VALUES ($1, $2, $3)
            RETURNING id")
            .bind(name)
            .bind(url)
            .bind(address_id)
            .fetch_one(&pool)
            .await;
        id_raw.unwrap().get::<i32, usize>(0)
    });
    return id;
}
pub fn create_experience(
    employing_entity_id: i32,
    title: &String,
    start_timestamptz: &chrono::DateTime<chrono::Utc>,
    end_timestamptz: &chrono::DateTime<chrono::Utc>,
    postal_address_id: i32,
) -> i32 {
    let rt = tokio::runtime::Runtime::new();
    let id: i32 = rt.expect("REASON").block_on(async {
        let pool = setup().await;
        let id_raw = sqlx::query(
            "INSERT INTO 
            \"experience\" (employing_entity_id, title, start_timestamptz, end_timestamptz, postal_address_id) 
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id")
            .bind(employing_entity_id)
            .bind(title)
            .bind(start_timestamptz)
            .bind(end_timestamptz)
            .bind(postal_address_id)
            .fetch_one(&pool)
            .await;
        id_raw.unwrap().get::<i32, usize>(0)
    });
    return id;
}
pub fn create_achivement(
    experience_id: i32, 
    short_description: &String, 
    defense: &String,
) -> i32 {
    let rt = tokio::runtime::Runtime::new();
    let id: i32 = rt.expect("REASON").block_on(async {
        let pool = setup().await;
        let id_raw = sqlx::query(
            "INSERT INTO 
            \"achievement\" (experience_id, short_description, defense) 
            VALUES ($1, $2, $3)
            RETURNING id")
            .bind(experience_id)
            .bind(short_description)
            .bind(defense)
            .fetch_one(&pool)
            .await;
        id_raw.unwrap().get::<i32, usize>(0)
    });
    return id;
}
pub fn create_achivement_variant(
    achievement_id: i32, 
    description: String, 
    defense: String,
) -> i32 {
    let rt = tokio::runtime::Runtime::new();
    let id: i32 = rt.expect("REASON").block_on(async {
        let pool = setup().await;
        let id_raw = sqlx::query(
            "INSERT INTO 
            \"achievement_variant\" (achievement_id, description, defense) 
            VALUES ($1, $2, $3)
            RETURNING id")
            .bind(achievement_id)
            .bind(description)
            .bind(defense)
            .fetch_one(&pool)
            .await;
        id_raw.unwrap().get::<i32, usize>(0)
    });
    return id;
}
pub fn create_project(
    start_timestamptz: chrono::DateTime<chrono::Utc>,
    end_timestamptz: chrono::DateTime<chrono::Utc>,
    name: String,
    url: String,
    postal_address_id: i32,

) {
    let rt = tokio::runtime::Runtime::new();
    let _: Result<(), std::io::Error> = Ok(rt.expect("REASON").block_on(async {
        let pool = setup().await;
        let _ = sqlx::query(
            "INSERT INTO 
            \"project\" (start_timestamptz, end_timestamptz, name, url, postal_address_id) 
            VALUES ($1, $2, $3)")
            .bind(start_timestamptz)
            .bind(end_timestamptz)
            .bind(name)
            .bind(url)
            .bind(postal_address_id)
            .execute(&pool)
            .await;
    }));
}
pub fn create_project_highlight(
    project_id: i32,
    short_description: String,
    defense: String,
) {
    let rt = tokio::runtime::Runtime::new();
    let _: Result<(), std::io::Error> = Ok(rt.expect("REASON").block_on(async {
        let pool = setup().await;
        let _ = sqlx::query(
            "INSERT INTO 
            \"project_highlight\" (project_id, short_description, defense) 
            VALUES ($1, $2, $3)")
            .bind(project_id)
            .bind(short_description)
            .bind(defense)
            .execute(&pool)
            .await;
    }));
}
pub fn create_project_highlight_variant(
    project_highlight_id: i32,
    description: String,
    defense: String,
) {
    let rt = tokio::runtime::Runtime::new();
    let _: Result<(), std::io::Error> = Ok(rt.expect("REASON").block_on(async {
        let pool = setup().await;
        let _ = sqlx::query(
            "INSERT INTO 
            \"project_highlight_variant\" (project_highlight_id, description, defense) 
            VALUES ($1, $2, $3)")
            .bind(project_highlight_id)
            .bind(description)
            .bind(defense)
            .execute(&pool)
            .await;
    }));
}
pub fn create_listing_host(
    name: String,
    url: String,
) {
    let rt = tokio::runtime::Runtime::new();
    let _: Result<(), std::io::Error> = Ok(rt.expect("REASON").block_on(async {
        let pool = setup().await;
        let _ = sqlx::query(
            "INSERT INTO 
            \"listing_host\" (name, url) 
            VALUES ($1, $2, $3)")
            .bind(name)
            .bind(url)
            .execute(&pool)
            .await;
    }));
}
pub fn create_listing(
    listing_host_id: i32,
    description: String,
    posted_timestamptz: chrono::DateTime<chrono::Utc>,
    recruiter_contact_id: i32,
) {
    let rt = tokio::runtime::Runtime::new();
    let _: Result<(), std::io::Error> = Ok(rt.expect("REASON").block_on(async {
        let pool = setup().await;
        let _ = sqlx::query(
            "INSERT INTO 
            \"listing\" (listing_host_id, description, posted_timestamptz, recruiter_contact_id) 
            VALUES ($1, $2, $3, $4)")
            .bind(listing_host_id)
            .bind(description)
            .bind(posted_timestamptz)
            .bind(recruiter_contact_id)
            .execute(&pool)
            .await;
    }));
}
pub fn create_application(
    start_timestamptz: chrono::DateTime<chrono::Utc>,
    submitted_timestamptz: chrono::DateTime<chrono::Utc>,
    listing_id: i32,
) {
    let rt = tokio::runtime::Runtime::new();
    let _: Result<(), std::io::Error> = Ok(rt.expect("REASON").block_on(async {
        let pool = setup().await;
        let _ = sqlx::query(
            "INSERT INTO 
            \"application\" (start_timestamptz, submitted_timestamptz, listing_id) 
            VALUES ($1, $2, $3)")
            .bind(start_timestamptz)
            .bind(submitted_timestamptz)
            .bind(listing_id)
            .execute(&pool)
            .await;
    }));
}
pub fn create_application_selection(
    application_id: i32,
    achievement_variant_id: i32,
) {
    let rt = tokio::runtime::Runtime::new();
    let _: Result<(), std::io::Error> = Ok(rt.expect("REASON").block_on(async {
        let pool = setup().await;
        let _ = sqlx::query(
            "INSERT INTO 
            \"application_selection\" (application_id, achievement_variant_id) 
            VALUES ($1, $2)")
            .bind(application_id)
            .bind(achievement_variant_id)
            .execute(&pool)
            .await;
    }));
}
pub fn create_application_question_answer(
    application_id: i32,
    question: String,
    answer: String,
) {
    let rt = tokio::runtime::Runtime::new();
    let _: Result<(), std::io::Error> = Ok(rt.expect("REASON").block_on(async {
        let pool = setup().await;
        let _ = sqlx::query(
            "INSERT INTO 
            \"application_question_answer\" (application_id, question, answer) 
            VALUES ($1, $2, $3)")
            .bind(application_id)
            .bind(question)
            .bind(answer)
            .execute(&pool)
            .await;
    }));
}
//********************** END OF NEW FUNCTIONS ************************************
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

