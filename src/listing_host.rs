
use crate::my_database;
use sqlx::Row;

// Organizes Postal Addresses in cache
// Including database access
#[derive(Clone, PartialEq, Debug)]
pub struct ListingHost {
    pub id: i32,
    pub name: String,
    pub url: String,
}
impl ListingHost {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::from(""),
            url: String::from(""),
        }
    }
    pub fn reset(&mut self) {
        self.id = 0;
        self.name = String::from("");
        self.url = String::from("");
    }
    pub fn fetch_many(offset: i32, max: i32) -> Vec<Self> {
        let expr = format!("
            SELECT 
                id,
                name,
                url,
            FROM
                listing_host
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
                name: row.get::<String, usize>(1),
                url: row.get::<String, usize>(2),
            };
            new_vec.push(element);
        }
        return new_vec;
    }
    pub fn fetch_by_id(id: i32) -> Self {
        let expr = format!("
            SELECT 
                id,
                name,
                url,
            FROM
                listing_host
            WHERE
                id = {}
            ", id);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        Self {
            id: row.get::<i32, usize>(0),
            name: row.get::<String, usize>(1),
            url: row.get::<String, usize>(2),
        }
    }
    // Send to db, get resulting id
    // put id in self.id and return it 
    pub fn insert_into_db(&mut self) -> i32 {
        let expr = format!("
            INSERT INTO listing_host 
                (
                name,
                url,
                ) 
            VALUES 
                ('{}', '{}')
            RETURNING id
            ", 
            self.name, 
            self.url);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        self.id = row.get::<i32, usize>(0);
        self.id
    }
}
