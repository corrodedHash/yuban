use rocket::Route;
use std::path::PathBuf;

use crate::StaticFilePath;
use rocket::response::NamedFile;
#[rocket::get("/post/<_path..>", rank = 8)]
fn post_route(_path: PathBuf, static_path: rocket::State<StaticFilePath>) -> Option<NamedFile> {
    let path = static_path.0.clone();
    NamedFile::open(path.join("index.html")).ok()
}
#[rocket::get("/newpost/<_path..>", rank = 8)]
fn newpost_route(_path: PathBuf, static_path: rocket::State<StaticFilePath>) -> Option<NamedFile> {
    let path = static_path.0.clone();
    NamedFile::open(path.join("index.html")).ok()
}

#[rocket::get("/correction/<_path..>", rank = 8)]
fn corr_route(_path: PathBuf, static_path: rocket::State<StaticFilePath>) -> Option<NamedFile> {
    let path = static_path.0.clone();
    NamedFile::open(path.join("index.html")).ok()
}

#[rocket::get("/newcorrection/<_path..>", rank = 8)]
fn newcorr_route(_path: PathBuf, static_path: rocket::State<StaticFilePath>) -> Option<NamedFile> {
    let path = static_path.0.clone();
    NamedFile::open(path.join("index.html")).ok()
}

#[rocket::get("/menu", rank = 8)]
fn menu_route(static_path: rocket::State<StaticFilePath>) -> Option<NamedFile> {
    let path = static_path.0.clone();
    NamedFile::open(path.join("index.html")).ok()
}
#[rocket::get("/admin", rank = 8)]
fn admin_route(static_path: rocket::State<StaticFilePath>) -> Option<NamedFile> {
    let path = static_path.0.clone();
    NamedFile::open(path.join("index.html")).ok()
}

pub fn get_fix_routes() -> impl Into<Vec<Route>> {
    rocket::routes![
        post_route,
        newpost_route,
        corr_route,
        newcorr_route,
        menu_route,
        admin_route,
    ]
}
