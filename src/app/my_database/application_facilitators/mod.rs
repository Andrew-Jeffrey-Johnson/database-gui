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
