
use std::collections::HashMap;
use crate::my_database;
use sqlx::Row;

// Organizes Postal Addresses in cache
// Including database access
#[derive(Clone, PartialEq, Debug, Default)]
pub struct EmployingEntity {
   pub id: i32,
   pub name: String,
   pub url: String,
   pub headquarters_postal_address_id: i32,
}
impl EmployingEntity {
    pub fn fetch(offset: i32, max: i32) -> HashMap<i32, Self> {
        let expr = format!("
            SELECT 
                id,
                name,
                url,
                headquarters_postal_address_id
            FROM
                employing_entity
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
                headquarters_postal_address_id: row.get::<i32, usize>(3),
            };
            new_vec.insert(element.id, element);
        }
        return new_vec;
    }
    // Send to db, get resulting id in self.id
    pub fn insert_into_db(&mut self) {
        let expr = format!("
            INSERT INTO employing_entity 
                (name, url, headquarters_postal_address_id) 
            VALUES 
                ('{}', '{}', {})
            RETURNING id
            ", self.name, self.url, self.headquarters_postal_address_id);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        self.id = row.get::<i32, usize>(0);
    }
}
