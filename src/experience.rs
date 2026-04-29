
use chrono::offset::TimeZone;
use chrono::offset::Utc;
use crate::my_database;
use crate::postal_address::PostalAddress;
//use crate::achievement::Achievement;

// Cache operations for Experience
// including database access
#[derive(Clone, PartialEq, Debug)]
struct Experience {
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
    /*
    pub fn reset(&mut self) {
        self.id = 0;
        self.employing_entity_id = 0;
        self.start_timestamptz = chrono::offset::Utc::now();
        self.end_timestamptz = chrono::offset::Utc::now();
        self.postal_address_id = 0;
    }
    pub fn fetch_all_from_db() -> Vec<Self> {
        let rows = my_database::fetch_all_from_postal_address();
        let mut addrs = Vec::<Self>::new();
        for row in rows {
            let addr = Self {
                id: row.0,
                name: row.1,
                line_1: row.2,
                line_2: row.3,
                line_3: row.4,
                city: row.5,
                state: row.6,
                zip_code: row.7,
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
    */
}
