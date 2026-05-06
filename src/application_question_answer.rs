
use crate::my_database;
use sqlx::Row;

// Organizes Postal Addresses in cache
// Including database access
#[derive(Clone, PartialEq, Debug)]
pub struct ApplicationQuestionAnswer {
    pub id: i32,
    pub application_id: i32,
    pub question: String,
    pub answer: String,
}
impl ApplicationQuestionAnswer {
    pub fn new() -> Self {
        Self {
            id: 0,
            application_id: 0,
            question: String::from(""),
            answer: String::from(""),
        }
    }
    pub fn reset(&mut self) {
        self.id = 0;
        self.application_id = 0;
        self.question = String::from("");
        self.answer = String::from("");
    }
    pub fn fetch_many(offset: i32, max: i32) -> Vec<Self> {
        let expr = format!("
            SELECT 
                id,
                application_id,
                question,
                answer,
            FROM
                application_question_answer
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
                application_id: row.get::<i32, usize>(1),
                question: row.get::<String, usize>(2),
                answer: row.get::<String, usize>(3),
            };
            new_vec.push(element);
        }
        return new_vec;
    }
    pub fn fetch_by_id(id: i32) -> Self {
        let expr = format!("
            SELECT 
                id,
                application_id,
                question,
                answer,
            FROM
                application_question_answer
            WHERE
                id = {}
            ", id);
        let rows: Vec<sqlx::postgres::PgRow> = my_database::fetch(&expr);
        let row = &rows[0];
        Self {
            id: row.get::<i32, usize>(0),
            application_id: row.get::<i32, usize>(1),
            question: row.get::<String, usize>(2),
            answer: row.get::<String, usize>(3),
        }
    }
    // Send to db, get resulting id
    // put id in self.id and return it 
    pub fn insert_into_db(&mut self) -> i32 {
        let expr = format!("
            INSERT INTO application_question_answer
                (
                application_id,
                question,
                answer,
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
        self.id
    }
}
