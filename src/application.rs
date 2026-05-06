
use crate::my_database;
use sqlx::Row;

// Organizes Postal Addresses in cache
// Including database access
#[derive(Clone, PartialEq, Debug)]
pub struct Application {
    pub id: i32,
    pub start_timestamptz: chrono::DateTime<chrono::Utc>,
    pub submitted_timestamptz: chrono::DateTime<chrono::Utc>,
    pub listing_id: i32,
}
impl Application {
    pub fn new() -> Self {
        Self {
            id: 0,
            start_timestamptz: chrono::offset::Utc::now(),
            submitted_timestamptz: chrono::offset::Utc::now(),
            listing_id: 0,
        }
    }
    pub fn reset(&mut self) {
        self.id = 0;
        self.start_timestamptz = chrono::offset::Utc::now();
        self.submitted_timestamptz = chrono::offset::Utc::now();
        self.listing_id = 0;
    }
    pub fn fetch_many(offset: i32, max: i32) -> Vec<Self> {
        let expr = format!("
            SELECT 
                id,
                start_timestamptz,
                submitted_timestamptz,
                listing_id,
            FROM
                application
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
                start_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(1),
                submitted_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(2),
                listing_id: row.get::<i32, usize>(3),
            };
            new_vec.push(element);
        }
        return new_vec;
    }
    pub fn fetch_by_id(id: i32) -> Self {
        let expr = format!("
            SELECT 
                id,
                start_timestamptz,
                submitted_timestamptz,
                listing_id,
            FROM
                application
            WHERE
                id = {}
            ", id);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        Self {
            id: row.get::<i32, usize>(0),
            start_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(1),
            submitted_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(2),
            listing_id: row.get::<i32, usize>(3),
        }
    }
    // Send to db, get resulting id
    // put id in self.id and return it 
    pub fn insert_into_db(&mut self) -> i32 {
        let expr = format!("
            INSERT INTO application 
                (
                start_timestamptz,
                submitted_timestamptz,
                listing_id,
                ) 
            VALUES 
                ('{}', '{}', '{}')
            RETURNING id
            ", 
            self.start_timestamptz, 
            self.submitted_timestamptz, 
            self.listing_id);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        self.id = row.get::<i32, usize>(0);
        self.id
    }
}
