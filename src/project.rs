
use crate::my_database;
use sqlx::Row;

// Organizes Postal Addresses in cache
// Including database access
#[derive(Clone, PartialEq, Debug)]
pub struct Project {
    pub id: i32,
    pub start_timestamptz: chrono::DateTime<chrono::Utc>,
    pub end_timestamptz: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub url: String,
    pub postal_address_id: i32,
}
impl Project {
    pub fn new() -> Self {
        Self {
            id: 0,
            start_timestamptz: chrono::offset::Utc::now(),
            end_timestamptz: chrono::offset::Utc::now(),
            name: String::from(""),
            url: String::from(""),
            postal_address_id: 0,
        }
    }
    pub fn reset(&mut self) {
        self.id = 0;
        self.start_timestamptz = chrono::offset::Utc::now();
        self.end_timestamptz = chrono::offset::Utc::now();
        self.name = String::from("");
        self.url = String::from("");
        self.postal_address_id = 0;
    }
    pub fn fetch_many(offset: i32, max: i32) -> Vec<Self> {
        let expr = format!("
            SELECT 
                id,
                start_timestamptz,
                end_timestamptz,
                name,
                url,
                postal_address_id,
            FROM
                project
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
                end_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(2),
                name: row.get::<String, usize>(3),
                url: row.get::<String, usize>(4),
                postal_address_id: row.get::<i32, usize>(5),
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
                end_timestamptz,
                name,
                url,
                postal_address_id,
            FROM
                project
            WHERE
                id = {}
            ", id);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        Self {
            id: row.get::<i32, usize>(0),
            start_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(1),
            end_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(2),
            name: row.get::<String, usize>(3),
            url: row.get::<String, usize>(4),
            postal_address_id: row.get::<i32, usize>(5),
        }
    }
    // Send to db, get resulting id
    // put id in self.id and return it 
    pub fn insert_into_db(&mut self) -> i32 {
        let expr = format!("
            INSERT INTO postal_address 
                (
                start_timestamptz,
                end_timestamptz,
                name,
                url,
                postal_address_id,
                ) 
            VALUES 
                ('{}', '{}', '{}', '{}', '{}')
            RETURNING id
            ", 
            self.start_timestamptz, 
            self.end_timestamptz, 
            self.name, 
            self.url, 
            self.postal_address_id);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        self.id = row.get::<i32, usize>(0);
        self.id
    }
}
