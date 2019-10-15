#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel; 
#[macro_use] extern crate serde;
extern crate chrono;

mod routes;
mod db;
mod schema;

use crate::routes::{ static_files, get };
use crate::db::{DB};

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(DB::fairing())
        .mount(
            "/",
            // 2.
            routes![
                static_files::file,
                static_files::pico8_file,
                get::index,
                get::read,
            ],
        )
}

fn main() {
    rocket().launch();
}