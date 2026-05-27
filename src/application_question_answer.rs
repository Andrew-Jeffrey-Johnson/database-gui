
use crate::my_database;
use sqlx::Row;
use std::collections::HashMap;

// Organizes Postal Addresses in cache
// Including database access
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ApplicationQuestionAnswer {
    pub id: i32,
    pub application_id: i32,
    pub question: String,
    pub answer: String,
}
impl ApplicationQuestionAnswer {
    pub fn fetch(offset: i32, max: i32) -> HashMap<i32, Self> {
        let expr = format!("
            SELECT 
                id,
                application_id,
                question,
                answer
            FROM
                application_question_answer
            ORDER BY
                id
            OFFSET {} ROWS
            FETCH FIRST {} ROWS ONLY
            ", offset, max);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let mut new_map = HashMap::<i32, Self>::with_capacity(rows.len());
        for row in rows {
            let element = Self {
                id: row.get::<i32, usize>(0),
                application_id: row.get::<i32, usize>(1),
                question: row.get::<String, usize>(2),
                answer: row.get::<String, usize>(3),
            };
            new_map.insert(element.id, element);
        }
        return new_map;
    }
    // Send to db, get resulting id in self.id
    pub fn insert_into_db(&mut self) {
        let expr = format!("
            INSERT INTO application_question_answer
                (
                application_id,
                question,
                answer
                ) 
            VALUES 
                ('{}', '{}', '{}')
            RETURNING id
            ", 
            self.application_id, 
            self.question, 
            self.answer);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        self.id = row.get::<i32, usize>(0);
    }
}
