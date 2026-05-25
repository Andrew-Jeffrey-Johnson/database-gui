
use std::collections::HashMap;
use crate::my_database;
use sqlx::Row;

// Organizes Postal Addresses in cache
// Including database access
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Listing {
    pub id: i32,
    pub listing_host_id: i32,
    pub description: String,
    pub posted_timestamptz: chrono::DateTime<chrono::Utc>,
    pub recruiter_contact_id: i32,
    pub url: String,
}
impl Listing {
    pub fn fetch(offset: i32, max: i32) -> HashMap<i32, Self> {
        let expr = format!("
            SELECT 
                id,
                listing_host_id,
                description,
                posted_timestamptz,
                recruiter_contact_id,
                url
            FROM
                listing
            ORDER BY
                id
            OFFSET {} ROWS
            FETCH FIRST {} ROWS ONLY
            ", offset, max);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let mut new_vec = HashMap::<i32, Self>::with_capacity(rows.len());
        for row in rows {
            let element = Self {
                id: row.get::<i32, usize>(0),
                listing_host_id: row.get::<i32, usize>(1),
                description: row.get::<String, usize>(2),
                posted_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(3),
                recruiter_contact_id: row.get::<i32, usize>(4),
                url: row.get::<String, usize>(5),
            };
            new_vec.insert(element.id, element);
        }
        return new_vec;
    }
    // Send to db, get resulting id in self.id
    pub fn insert_into_db(&mut self) {
        let expr = format!("
            INSERT INTO listing
                (
                listing_host_id,
                description,
                posted_timestamptz,
                recruiter_contact_id,
                url
                ) 
            VALUES 
                ('{}', '{}', '{}', '{}', '{}')
            RETURNING id
            ", 
            self.listing_host_id, 
            self.description, 
            self.posted_timestamptz, 
            self.recruiter_contact_id,
            self.url,
        );
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        self.id = row.get::<i32, usize>(0);
    }
}
