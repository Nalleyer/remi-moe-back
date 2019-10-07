#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use crate::routes::{ static_files, get };
mod routes;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            // 2.
            routes![
                static_files::file,
                static_files::pico8_file,
                get::index,
            ],
        )
}

fn main() {
    rocket().launch();
}
/*
#[get("/")]
fn index() -> &'static str {
    "Hello remi-moe"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

*/