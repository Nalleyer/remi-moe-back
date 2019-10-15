use chrono::naive::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use crate::schema::games;

#[derive(Queryable, Deserialize, Serialize)]
pub struct Game {
    pub id: u64,
    pub title: String,
    pub link: String,
    pub create_time: NaiveDateTime,
}

#[derive(Insertable, Serialize)]
#[table_name="games"]
pub struct NewGame {
    pub title: String,
    pub link: String,
    pub create_time: NaiveDateTime,
}

impl Game {
    pub fn read(conn: &MysqlConnection) -> Vec<Self> {
        games::table.load::<Game>(conn).unwrap()
    }
}