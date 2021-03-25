#![feature(proc_macro_hygiene, decl_macro)]
#![feature(bool_to_option)]

mod auth;
mod db;

use std::path::PathBuf;

use auth::AuthorizedUser;
use db::YubanDatabase;
use rocket::{
    http::{Cookie, Cookies, Status},
    request::FromParam,
    response::NamedFile,
    State,
};

#[derive(rocket::FromForm, serde::Deserialize)]
struct LoginData {
    username: String,
    password: String,
}
#[derive(Debug, Clone, serde::Deserialize)]
struct ArticlePost {
    original: String,
    correction: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Posts(Vec<ArticlePost>);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LangCode(String);

impl<'r> FromParam<'r> for LangCode {
    type Error = ();

    fn from_param(param: &'r rocket::http::RawStr) -> Result<Self, Self::Error> {
        if param.is_empty() {
            return Err(());
        }
        if param.len() != 2 {
            return Err(());
        }
        Ok(Self(param.as_str().to_owned()))
    }
}

#[rocket::post("/logindata", data = "<data>")]
fn login_post(
    data: rocket_contrib::json::Json<LoginData>,
    mut cookies: Cookies,
    db: State<db::YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, Status> {
    if let Ok(user_id) = db.check_login(&data.username, &data.password) {
        let token = match db.add_token(user_id) {
            Ok(t) => t,
            Err(_) => return Err(Status::BadGateway),
        };
        let str_token = base64::encode(token.as_ref());
        cookies.add(
            Cookie::build("token", str_token)
                .secure(true)
                .same_site(rocket::http::SameSite::Strict)
                .permanent()
                .path("/")
                .finish(),
        );
        cookies.add(
            Cookie::build("username", data.username.clone())
                .secure(true)
                .same_site(rocket::http::SameSite::Strict)
                .permanent()
                .path("/")
                .finish(),
        );
        return Ok(rocket::response::content::Json("true".to_owned()));
    }
    Ok(rocket::response::content::Json("false".to_owned()))
}

#[rocket::get("/testtoken")]
fn test_token(_user: AuthorizedUser) -> Status {
    Status::Accepted
}

#[rocket::put("/newpost/<langcode>", data = "<data>")]
fn new_post(
    user: AuthorizedUser,
    data: String,
    langcode: LangCode,
    db: State<db::YubanDatabase>,
) -> Result<String, Status> {
    db.add_new_thread(&user.username, &data, &langcode.0)
        .map_err(|_| Status::InternalServerError)
        .map(|(thread_id, post_id)| format!("{{'thread_id':{}, 'post_id':{}}}", thread_id, post_id))
}

#[rocket::put("/addpost/<thread_id>/<langcode>", data = "<data>")]
fn new_translation(
    user: AuthorizedUser,
    data: String,
    thread_id: u64,
    langcode: LangCode,
    db: State<db::YubanDatabase>,
) -> Result<String, Status> {
    db.add_post(&user.username, &data, thread_id, &langcode.0)
        .map_err(|_| Status::InternalServerError)
        .map(|x| x.to_string())
}

#[rocket::put("/addcorrection/<origpostid>", data = "<data>")]
fn new_correction(
    user: AuthorizedUser,
    data: String,
    origpostid: u64,
    db: State<db::YubanDatabase>,
) -> Result<String, Status> {
    db.add_correction(&user.username, &data, origpostid)
        .map_err(|_| Status::InternalServerError)
        .map(|x| x.to_string())
}

#[rocket::get("/posts")]
fn article_posts(
    _user: AuthorizedUser,
    db: State<YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, Status> {
    let posts = db
        .list_original_posts()
        .map_err(|_| Status::InternalServerError)?;
    let json_posts = serde_json::to_string(&posts).map_err(|_| Status::InternalServerError)?;
    Ok(rocket::response::content::Json(json_posts))
}

#[rocket::get("/post/<id>")]
fn single_post(
    id: usize,
    _user: AuthorizedUser,
    db: State<YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, Status> {
    let post = db.get_post(id).map_err(|_| Status::InternalServerError)?;
    let json_post = serde_json::to_string(&post).map_err(|_| Status::InternalServerError)?;
    Ok(rocket::response::content::Json(json_post))
}

#[rocket::get("/post/<path..>", rank = 8)]
fn post_route_fix(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/webpage/dist"))
            .join("index.html"),
    )
    .ok()
}
#[rocket::get("/newpost/<path..>", rank = 8)]
fn newpost_route_fix(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/webpage/dist"))
            .join("index.html"),
    )
    .ok()
}

#[rocket::get("/correction/<path..>", rank = 8)]
fn corr_route_fix(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/webpage/dist"))
            .join("index.html"),
    )
    .ok()
}

#[rocket::get("/newcorrection/<path..>", rank = 8)]
fn newcorr_route_fix(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/webpage/dist"))
            .join("index.html"),
    )
    .ok()
}

#[rocket::get("/menu", rank = 8)]
fn menu_route_fix() -> Option<NamedFile> {
    NamedFile::open(
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/webpage/dist"))
            .join("index.html"),
    )
    .ok()
}

fn main() {
    let db = db::YubanDatabase::new().expect("Could not open database");
    match db.new_login("me", "secret") {
        Ok(_) => {}
        Err(err) => {
            dbg!(err);
        }
    }
    use rocket::config::{Config, Environment};

    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(8000)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .manage(db)
        .mount(
            "/api",
            rocket::routes![
                login_post,
                article_posts,
                test_token,
                single_post,
                new_post,
                new_translation,
                new_correction
            ],
        )
        .mount(
            "/",
            rocket::routes![
                post_route_fix,
                newpost_route_fix,
                corr_route_fix,
                newcorr_route_fix,
                menu_route_fix
            ],
        )
        .mount(
            "/",
            rocket_contrib::serve::StaticFiles::from(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/webpage/dist"
            )),
        )
        .launch();
}
