
use crate::my_database;
use sqlx::Row;

// Organizes Postal Addresses in cache
// Including database access
#[derive(Clone, PartialEq, Debug)]
pub struct Listing {
    pub id: i32,
    pub listing_host_id: i32,
    pub description: String,
    pub posted_timestamptz: chrono::DateTime<chrono::Utc>,
    pub recruiter_contact_id: i32,
}
impl Listing {
    pub fn new() -> Self {
        Self {
            id: 0,
            listing_host_id: 0,
            description: String::from(""),
            posted_timestamptz: chrono::offset::Utc::now(),
            recruiter_contact_id: 0,
        }
    }
    pub fn reset(&mut self) {
        self.id = 0;
        self.listing_host_id = 0;
        self.description = String::from("");
        self.posted_timestamptz = chrono::offset::Utc::now();
        self.recruiter_contact_id = 0;
    }
    pub fn fetch_many(offset: i32, max: i32) -> Vec<Self> {
        let expr = format!("
            SELECT 
                id,
                listing_host_id,
                description,
                posted_timestamptz,
                recruiter_contact_id
            FROM
                listing
            ORDER BY
                id
            OFFSET {} ROWS
            FETCH FIRST {} ROWS ONLY
            ", offset, max);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let mut new_vec = Vec::<Self>::with_capacity(rows.len());
        for row in rows {
            let element = Self {
                id: row.get::<i32, usize>(0),
                listing_host_id: row.get::<i32, usize>(1),
                description: row.get::<String, usize>(2),
                posted_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(3),
                recruiter_contact_id: row.get::<i32, usize>(4),
            };
            new_vec.push(element);
        }
        return new_vec;
    }
    pub fn fetch_by_id(id: i32) -> Self {
        let expr = format!("
            SELECT 
                id,
                listing_host_id,
                description,
                posted_timestamptz,
                recruiter_contact_id
            FROM
                listing
            WHERE
                id = {}
            ", id);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        Self {
            id: row.get::<i32, usize>(0),
            listing_host_id: row.get::<i32, usize>(1),
            description: row.get::<String, usize>(2),
            posted_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(3),
            recruiter_contact_id: row.get::<i32, usize>(4),
        }
    }
    // Send to db, get resulting id
    // put id in self.id and return it 
    pub fn insert_into_db(&mut self) -> i32 {
        let expr = format!("
            INSERT INTO listing
                (
                listing_host_id,
                description,
                posted_timestamptz,
                recruiter_contact_id
                ) 
            VALUES 
                ('{}', '{}', '{}', '{}')
            RETURNING id
            ", 
            self.listing_host_id, 
            self.description, 
            self.posted_timestamptz, 
            self.recruiter_contact_id);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        self.id = row.get::<i32, usize>(0);
        self.id
    }
}
