
use crate::my_database;
use sqlx::Row;

// Organizes Postal Addresses in cache
// Including database access
#[derive(Clone, PartialEq, Debug)]
pub struct PostalAddress {
    pub id: i32,
    pub name: String,
    pub line_1: String,
    pub line_2: String,
    pub line_3: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
}
impl PostalAddress {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::from(""),
            line_1: String::from(""),
            line_2: String::from(""),
            line_3: String::from(""),
            city: String::from(""),
            state: String::from(""),
            zip_code: String::from(""),
        }
    }
    pub fn reset(&mut self) {
        self.id = 0;
        self.name = String::from("");
        self.line_1 = String::from("");
        self.line_2 = String::from("");
        self.line_3 = String::from("");
        self.city = String::from("");
        self.state = String::from("");
        self.zip_code = String::from("");
    }
    pub fn fetch_many(offset: i32, max: i32) -> Vec<Self> {
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
        let mut new_vec = Vec::<Self>::with_capacity(rows.len());
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
                line_1,
                line_2,
                line_3,
                city,
                state,
                zip_code
            FROM
                postal_address
            WHERE
                id = {}
            ", id);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        Self {
            id: row.get::<i32, usize>(0),
            name: row.get::<String, usize>(1),
            line_1: row.get::<String, usize>(2),
            line_2: row.get::<String, usize>(3),
            line_3: row.get::<String, usize>(4),
            city: row.get::<String, usize>(5),
            state: row.get::<String, usize>(6),
            zip_code: row.get::<String, usize>(7),
        }
    }
    // Send to db, get resulting id
    // put id in self.id and return it 
    pub fn insert_into_db(&mut self) -> i32 {
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
        self.id
    }
}
