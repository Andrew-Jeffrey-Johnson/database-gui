
use crate::my_database;
use sqlx::Row;

// Organizes Postal Addresses in cache
// Including database access
#[derive(Clone, PartialEq, Debug)]
pub struct EmployingEntity {
   pub id: i32,
   pub name: String,
   pub url: String,
   pub headquarters_postal_address_id: i32,
}
impl EmployingEntity {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::from(""),
            url: String::from(""),
            headquarters_postal_address_id: 0,
        }
    }
    pub fn reset(&mut self) {
        self.id = 0;
        self.name = String::from("");
        self.url = String::from("");
        self.headquarters_postal_address_id = 0;
    }
    pub fn fetch_many(offset: i32, max: i32) -> Vec<Self> {
        let expr = format!("
            SELECT 
                id,
                name,
                url,
                headquarters_postal_address_id,
            FROM
                employing_entity
            ORDER BY
                id
            OFFSET {} ROWS
            FETCH FIRST {} ROWS ONLY
            ", offset, max);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let mut addrs = Vec::<Self>::with_capacity(rows.len());
        for row in rows {
            let addr = Self {
                id: row.get::<i32, usize>(0),
                name: row.get::<String, usize>(1),
                url: row.get::<String, usize>(2),
                headquarters_postal_address_id: row.get::<i32, usize>(3),
            };
            addrs.push(addr);
        }
        return addrs;
    }
    pub fn fetch_by_id(id: i32) -> Self {
        let expr = format!("
            SELECT 
                id,
                name,
                url,
                headquarters_postal_address_id,
            FROM
                employing_entity
            WHERE
                id = {}
            ", id);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        Self {
            id: row.get::<i32, usize>(0),
            name: row.get::<String, usize>(1),
            url: row.get::<String, usize>(2),
            headquarters_postal_address_id: row.get::<i32, usize>(3),
        }
    }
    // Send to db, get resulting id
    // put id in self.id and return it 
    pub fn insert_into_db(&mut self) -> i32 {
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
        self.id
    }
}
