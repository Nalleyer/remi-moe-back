use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/remi/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("front/dist/").join(file)).ok()
}

#[get("/pico8/<file..>")]
pub fn pico8_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("pico8/").join(file)).ok()
}