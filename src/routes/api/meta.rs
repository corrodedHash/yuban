lazy_static::lazy_static! {
    static ref VERSION_JSON: String = format!("\"{}\"", env!("CARGO_PKG_VERSION"));
}

#[rocket::get("/version")]
pub fn get_version() -> rocket::response::content::Json<&'static str> {
    rocket::response::content::Json(&VERSION_JSON)
}

pub fn get_meta_routes() -> impl Into<Vec<rocket::Route>> {
    rocket::routes![get_version]
}
