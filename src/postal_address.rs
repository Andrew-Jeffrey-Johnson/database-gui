
use std::collections::HashMap;
use crate::my_database;
use sqlx::Row;

// Organizes Postal Addresses in cache
// Including database access
#[derive(Clone, PartialEq, Debug, Default)]
pub struct PostalAddress {
    // Database rows
    pub id: i32,
    pub name: String,
    pub line_1: String,
    pub line_2: String,
    pub line_3: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,

    // Application variables
    pub is_selected: bool,
    pub priority: i32,
}

impl PostalAddress {
    // Fetch all rows where offset <= id <= offset+max
    pub fn fetch(offset: i32, max: i32) -> HashMap<i32, Self> {
        let expr = format!("
            SELECT 
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
            OFFSET {} ROWS
            FETCH FIRST {} ROWS ONLY
            ", offset, max);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let mut new_vec = HashMap::<i32, Self>::with_capacity(rows.len());
        for row in rows {
            let element = Self {
                id: row.get::<i32, usize>(0),
                name: row.get::<String, usize>(1),
                line_1: row.get::<String, usize>(2),
                line_2: row.get::<String, usize>(3),
                line_3: row.get::<String, usize>(4),
                city: row.get::<String, usize>(5),
                state: row.get::<String, usize>(6),
                zip_code: row.get::<String, usize>(7),
                ..Default::default()
            };
            new_vec.insert(element.id, element);
        }
        return new_vec;
    }
    // Send to db, get resulting id
    // put id in self.id 
    pub fn insert_into_db(&mut self) {
        let expr = format!("
            INSERT INTO postal_address 
                (
                name, 
                line_1, 
                line_2, 
                line_3, 
                city, 
                state, 
                zip_code
                ) 
            VALUES 
                ('{}', '{}', '{}', '{}', '{}', '{}', '{}')
            RETURNING id
            ", 
            self.name, 
            self.line_1, 
            self.line_2, 
            self.line_3, 
            self.city, 
            self.state, 
            self.zip_code);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        self.id = row.get::<i32, usize>(0);
    }
}
