
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
    pub fn fetch_all_from_db() -> Vec<Self> {
        //let rows = my_database::fetch_all_from_postal_address();
        let expr = String::from("
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
            OFFSET 0 ROWS
            FETCH FIRST 100 ROWS ONLY
            ");
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch_all(&expr);
        let mut addrs = Vec::<Self>::with_capacity(rows.len());
        for row in rows {
            let addr = Self {
                id: row.get::<i32, usize>(0),
                name: row.get::<String, usize>(1),
                line_1: row.get::<String, usize>(2),
                line_2: row.get::<String, usize>(3),
                line_3: row.get::<String, usize>(4),
                city: row.get::<String, usize>(5),
                state: row.get::<String, usize>(6),
                zip_code: row.get::<String, usize>(7),
            };
            addrs.push(addr);
        }
        return addrs;
    }
    pub fn fetch_one_from_db_with_id(id: i32) -> Self {
        let row = my_database::fetch_one_from_postal_address(id);
        Self {
            id: row.0,
            name: row.1,
            line_1: row.2,
            line_2: row.3,
            line_3: row.4,
            city: row.5,
            state: row.6,
            zip_code: row.7,
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
    // Send to db, get resulting id
    // put id in self.id and return it 
    pub fn send_to_db_as_new_row(&mut self) -> i32 {
        self.id = my_database::create_postal_address(
            &self.name, 
            &self.line_1, 
            &self.line_2, 
            &self.line_3, 
            &self.city, 
            &self.state, 
            &self.zip_code
        );
        self.id
    }
}
