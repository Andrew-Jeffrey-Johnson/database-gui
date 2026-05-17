
use std::collections::HashMap;
use sqlx::Row;
use crate::my_database;

// Cache operations for Experience
// including database access
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Experience {
    pub id: i32,
    pub employing_entity_id: i32,
    pub start_timestamptz: chrono::DateTime<chrono::Utc>,
    pub end_timestamptz: chrono::DateTime<chrono::Utc>,
    pub postal_address_id: i32,
    pub title: String,
}
impl Experience {
    pub fn fetch(offset: i32, max: i32) -> HashMap<i32, Self> {
        let expr = format!("
            SELECT 
                id,
                employing_entity_id,
                start_timestamptz,
                end_timestamptz,
                postal_address_id,
                title
            FROM
                experience
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
                employing_entity_id: row.get::<i32, usize>(1),
                start_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(2),
                end_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(3),
                postal_address_id: row.get::<i32, usize>(4),
                title: row.get::<String, usize>(5),
            };
            new_vec.insert(element.id, element);
        }
        return new_vec;
    }
    // Send to db, get resulting id in self.id
    pub fn insert_into_db(&mut self) {
        let expr = format!("
            INSERT INTO postal_address 
                (
                employing_entity_id, 
                start_timestamptz, 
                end_timestamptz, 
                postal_address_id,
                title
                ) 
            VALUES 
                ('{}', '{}', '{}', '{}', '{}')
            RETURNING id
            ", 
            self.employing_entity_id, 
            self.start_timestamptz, 
            self.end_timestamptz, 
            self.postal_address_id,
            self.title);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        self.id = row.get::<i32, usize>(0);
    }
}
