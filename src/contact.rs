
use crate::my_database;
use sqlx::Row;

// Organizes Postal Addresses in cache
// Including database access
#[derive(Clone, PartialEq, Debug)]
pub struct Contact {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub phone_number: String,
    pub url: String,
    pub address_id: i32,
}
impl Contact {
    pub fn new() -> Self {
        Self {
            id: 0,
            first_name: String::from(""),
            last_name: String::from(""),
            username: String::from(""),
            email: String::from(""),
            phone_number: String::from(""),
            url: String::from(""),
            address_id: 0,
        }
    }
    pub fn reset(&mut self) {
        self.id = 0;
        self.first_name = String::from("");
        self.last_name = String::from("");
        self.username = String::from("");
        self.email = String::from("");
        self.phone_number = String::from("");
        self.url = String::from("");
        self.address_id = 0;
    }
    pub fn fetch_many(offset: i32, max: i32) -> Vec<Self> {
        let expr = format!("
            SELECT 
                id,
                first_name,
                last_name,
                username,
                email,
                phone_number,
                url,
                address_id
            FROM
                contact
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
                first_name: row.get::<String, usize>(1),
                last_name: row.get::<String, usize>(2),
                username: row.get::<String, usize>(3),
                email: row.get::<String, usize>(4),
                phone_number: row.get::<String, usize>(5),
                url: row.get::<String, usize>(6),
                address_id: row.get::<i32, usize>(7),
            };
            new_vec.push(element);
        }
        return new_vec;
    }
    pub fn fetch_by_id(id: i32) -> Self {
        let expr = format!("
            SELECT 
                id,
                first_name,
                last_name,
                username,
                email,
                phone_number,
                url,
                address_id
            FROM
                contact
            WHERE
                id = {}
            ", id);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        Self {
            id: row.get::<i32, usize>(0),
            first_name: row.get::<String, usize>(1),
            last_name: row.get::<String, usize>(2),
            username: row.get::<String, usize>(3),
            email: row.get::<String, usize>(4),
            phone_number: row.get::<String, usize>(5),
            url: row.get::<String, usize>(6),
            address_id: row.get::<i32, usize>(7),
        }
    }
    // Send to db, get resulting id
    // put id in self.id and return it 
    pub fn insert_into_db(&mut self) -> i32 {
        let expr = format!("
            INSERT INTO postal_address 
                (
                first_name,
                last_name,
                username,
                email,
                phone_number,
                url,
                address_id
                )
            VALUES 
                ('{}', '{}', '{}', '{}', '{}', '{}', '{}')
            RETURNING id
            ", 
            self.first_name, 
            self.last_name, 
            self.username, 
            self.email, 
            self.phone_number, 
            self.url, 
            self.address_id);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        self.id = row.get::<i32, usize>(0);
        self.id
    }
}
