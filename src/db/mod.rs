// pub mod models;
use rocket_contrib::databases::diesel;

#[database("sql_db")]
pub struct DB(diesel::MysqlConnection);


pub mod models;