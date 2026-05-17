
use std::collections::HashMap;
use crate::my_database;
use sqlx::Row;

// Organizes Postal Addresses in cache
// Including database access
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Achievement {
    // For table
    pub id: i32,
    pub experience_id: i32,
    pub short_description: String,
    pub defense: String,
    // For egui app
    pub is_selected: bool, 
}
impl Achievement {
    pub fn fetch_using_experience(offset: i32, max: i32, experience_id: i32) -> HashMap<i32, Self> {
        let expr = format!("
            SELECT 
                id,
                experience_id,
                short_description,
                defense
            FROM
                achievement
            WHERE
                experience_id = {}
            ORDER BY
                id
            OFFSET {} ROWS
            FETCH FIRST {} ROWS ONLY
            ", experience_id, offset, max);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let mut new_vec = HashMap::<i32, Self>::with_capacity(rows.len());
        for row in rows {
            let element = Self {
                id: row.get::<i32, usize>(0),
                experience_id: row.get::<i32, usize>(1),
                short_description: row.get::<String, usize>(2),
                defense: row.get::<String, usize>(3),
                ..Default::default()
            };
            new_vec.insert(element.id, element);
        }
        return new_vec;
    }
    // Send to db, get resulting id
    // put id in self.id and return it 
    pub fn insert_into_db(&mut self) {
        let expr = format!("
            INSERT INTO achievement 
                ( 
                experience_id,
                short_description,
                defense
                )
            VALUES 
                ('{}', '{}', '{}')
            RETURNING id
            ", 
            self.experience_id, 
            self.short_description, 
            self.defense);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        self.id = row.get::<i32, usize>(0);
    }
}
