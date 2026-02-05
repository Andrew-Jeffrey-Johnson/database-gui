#[derive(sqlx::FromRow)]
pub struct Description {
    pub id: i32,
    pub original_text: String,
}

pub async fn create_description(
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
