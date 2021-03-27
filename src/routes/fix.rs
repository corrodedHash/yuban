use rocket::Route;
use std::path::PathBuf;

use rocket::response::NamedFile;

#[rocket::get("/post/<path..>", rank = 8)]
fn post_route(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/webpage/dist"))
            .join("index.html"),
    )
    .ok()
}
#[rocket::get("/newpost/<path..>", rank = 8)]
fn newpost_route(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/webpage/dist"))
            .join("index.html"),
    )
    .ok()
}

#[rocket::get("/correction/<path..>", rank = 8)]
fn corr_route(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/webpage/dist"))
            .join("index.html"),
    )
    .ok()
}

#[rocket::get("/newcorrection/<path..>", rank = 8)]
fn newcorr_route(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/webpage/dist"))
            .join("index.html"),
    )
    .ok()
}

#[rocket::get("/menu", rank = 8)]
fn menu_route() -> Option<NamedFile> {
    NamedFile::open(
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/webpage/dist"))
            .join("index.html"),
    )
    .ok()
}

pub fn get_fix_routes() -> impl Into<Vec<Route>> {
    rocket::routes![
        post_route,
        newpost_route,
        corr_route,
        newcorr_route,
        menu_route
    ]
}
