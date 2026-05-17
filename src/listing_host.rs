
use std::collections::HashMap;
use crate::my_database;
use sqlx::Row;

// Organizes Postal Addresses in cache
// Including database access
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ListingHost {
    pub id: i32,
    pub name: String,
    pub url: String,
}
impl ListingHost {
    pub fn fetch(offset: i32, max: i32) -> HashMap<i32, Self> {
        let expr = format!("
            SELECT 
                id,
                name,
                url
            FROM
                listing_host
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
                name: row.get::<String, usize>(1),
                url: row.get::<String, usize>(2),
            };
            new_vec.insert(element.id, element);
        }
        return new_vec;
    }
    // Send to db, get resulting id in self.id
    pub fn insert_into_db(&mut self) {
        let expr = format!("
            INSERT INTO listing_host 
                (
                name,
                url
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
    }
}
