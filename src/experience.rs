
use sqlx::Row;
use crate::my_database;

// Cache operations for Experience
// including database access
#[derive(Clone, PartialEq, Debug)]
pub struct Experience {
    pub id: i32,
    pub employing_entity_id: i32,
    pub start_timestamptz: chrono::DateTime<chrono::Utc>,
    pub end_timestamptz: chrono::DateTime<chrono::Utc>,
    pub postal_address_id: i32,
}
impl Experience {
    pub fn new() -> Self {
        Self {
            id: 0,
            employing_entity_id: 0,
            start_timestamptz: chrono::offset::Utc::now(),
            end_timestamptz: chrono::offset::Utc::now(),
            postal_address_id: 0,
        }
    }
    pub fn reset(&mut self) {
        self.id = 0;
        self.employing_entity_id = 0;
        self.start_timestamptz = chrono::offset::Utc::now();
        self.end_timestamptz = chrono::offset::Utc::now();
        self.postal_address_id = 0;
    }
    pub fn fetch_many(offset: i32, max: i32) -> Vec<Self> {
        let expr = format!("
            SELECT 
                id,
                employing_entity_id,
                start_timestamptz,
                end_timestamptz,
                postal_address_id,
            FROM
                experience
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
                employing_entity_id: row.get::<i32, usize>(1),
                start_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(2),
                end_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(3),
                postal_address_id: row.get::<i32, usize>(4),
            };
            new_vec.push(element);
        }
        return new_vec;
    }
    pub fn fetch_by_id(id: i32) -> Self {
        let expr = format!("
            SELECT 
                id,
                employing_entity_id,
                start_timestamptz,
                end_timestamptz,
                postal_address_id,
            FROM
                experience
            WHERE
                id = {}
            ", id);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        Self {
            id: row.get::<i32, usize>(0),
            employing_entity_id: row.get::<i32, usize>(1),
            start_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(2),
            end_timestamptz: row.get::<chrono::DateTime<chrono::Utc>, usize>(3),
            postal_address_id: row.get::<i32, usize>(4),
        }
    }
    // Send to db, get resulting id
    // put id in self.id and return it 
    pub fn insert_into_db(&mut self) -> i32 {
        let expr = format!("
            INSERT INTO postal_address 
                (employing_entity_id, 
                start_timestamptz, 
                end_timestamptz, 
                postal_address_id) 
            VALUES 
                ('{}', '{}', '{}', '{}')
            RETURNING id
            ", 
            self.employing_entity_id, 
            self.start_timestamptz, 
            self.end_timestamptz, 
            self.postal_address_id);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        self.id = row.get::<i32, usize>(0);
        self.id
    }
}
