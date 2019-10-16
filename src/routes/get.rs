use std::io;
use rocket::response::{NamedFile};
use rocket_contrib::{
    json::{Json, JsonValue},
};

use crate::db::{
    DB,
    models::{Game},
};

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("front/dist/index.html")
}

#[get("/api/game_list")]
pub fn read(connection: DB) -> Json<JsonValue> {
    Json(json!(Game::read(&connection)))
}