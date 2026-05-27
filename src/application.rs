
use std::collections::HashMap;
use crate::my_database;
use sqlx::Row;

// Organizes Postal Addresses in cache
// Including database access
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Application {
    // Database columns
    pub id: i32,
    pub start_timestamptz: chrono::DateTime<chrono::Utc>,
    pub submitted_timestamptz: chrono::DateTime<chrono::Utc>,
    pub listing_id: i32,
    pub resume: String,
    // App state
    pub is_started: bool,
}
impl Application {
    pub fn fetch(offset: i32, max: i32) -> HashMap<i32, Self> {
        let expr = format!("
            SELECT 
                id,
                start_timestamptz,
                submitted_timestamptz,
                listing_id,
                resume
            FROM
                application
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
                start_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(1),
                submitted_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(2),
                listing_id: row.get::<i32, usize>(3),
                resume: row.get::<String, usize>(4),
                ..Default::default()
            };
            new_vec.insert(element.id, element);
        }
        return new_vec;
    }
    // Send to db, get resulting id in self.id
    pub fn insert_into_db(&mut self) {
        let expr = format!("
            INSERT INTO application 
                (
                start_timestamptz,
                submitted_timestamptz,
                listing_id,
                resume
                ) 
            VALUES 
                ('{}', '{}', '{}', '{}')
            RETURNING id
            ", 
            self.start_timestamptz, 
            self.submitted_timestamptz, 
            self.listing_id,
            self.resume,
        );
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        self.id = row.get::<i32, usize>(0);
    }
}
