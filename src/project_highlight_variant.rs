
use crate::my_database;
use sqlx::Row;

// Organizes Postal Addresses in cache
// Including database access
#[derive(Clone, PartialEq, Debug)]
pub struct ProjectHighlightVariant {
    pub id: i32,
    pub project_highlight_id: i32,
    pub description: String,
    pub defense: String,
}
impl ProjectHighlightVariant {
    pub fn new() -> Self {
        Self {
            id: 0,
            project_highlight_id: 0,
            description: String::from(""),
            defense: String::from(""),
        }
    }
    pub fn reset(&mut self) {
        self.id = 0;
        self.project_highlight_id = 0;
        self.description = String::from("");
        self.defense = String::from("");
    }
    pub fn fetch_many(offset: i32, max: i32) -> Vec<Self> {
        let expr = format!("
            SELECT 
                id,
                project_highlight_id,
                description,
                defense,
            FROM
                project_highlight_variant
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
                project_highlight_id: row.get::<i32, usize>(1),
                description: row.get::<String, usize>(2),
                defense: row.get::<String, usize>(3),
            };
            new_vec.push(element);
        }
        return new_vec;
    }
    pub fn fetch_by_id(id: i32) -> Self {
        let expr = format!("
            SELECT 
                id,
                project_highlight_id,
                description,
                defense,
            FROM
                project_highlight_variant
            WHERE
                id = {}
            ", id);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        Self {
            id: row.get::<i32, usize>(0),
            project_highlight_id: row.get::<i32, usize>(1),
            description: row.get::<String, usize>(2),
            defense: row.get::<String, usize>(3),
        }
    }
    // Send to db, get resulting id
    // put id in self.id and return it 
    pub fn insert_into_db(&mut self) -> i32 {
        let expr = format!("
            INSERT INTO project_highlight_variant
                ( 
                project_highlight_id,
                description,
                defense,
                )
            VALUES 
                ('{}', '{}', '{}')
            RETURNING id
            ", 
            self.project_highlight_id, 
            self.description, 
            self.defense);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        self.id = row.get::<i32, usize>(0);
        self.id
    }
}
