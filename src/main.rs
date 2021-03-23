#![feature(proc_macro_hygiene, decl_macro)]
#![feature(bool_to_option)]

mod auth;
mod db;

use auth::AuthorizedUser;
use db::YubanDatabase;
use rocket::{
    http::{Cookie, Cookies, Status},
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

#[rocket::post("/logindata", data = "<data>")]
fn login_post(
    data: rocket_contrib::json::Json<LoginData>,
    mut cookies: Cookies,
    db: State<db::YubanDatabase>,
) -> rocket::http::Status {
    if let Ok(user_id) = db.check_login(&data.username, &data.password) {
        let token = match db.add_token(user_id) {
            Ok(t) => t,
            Err(_) => return Status::BadGateway,
        };
        let str_token = base64::encode(token.as_ref());
        cookies.add(
            Cookie::build("token", str_token)
                .secure(true)
                .same_site(rocket::http::SameSite::Strict)
                .finish(),
        );
        cookies.add(
            Cookie::build("username", data.username.clone())
                .secure(true)
                .same_site(rocket::http::SameSite::Strict)
                .finish(),
        );
        return rocket::http::Status::Accepted;
    }
    rocket::http::Status::Forbidden
}

#[rocket::get("/testtoken")]
fn test_token(_user: AuthorizedUser) -> rocket::http::Status {
    Status::Accepted
}

#[rocket::put("/newpost", data = "<data>")]
fn new_post(
    user: AuthorizedUser,
    data: String,
    db: State<db::YubanDatabase>,
) -> Result<String, rocket::http::Status> {
    db.add_post(&user.username, &data)
        .map_err(|_| Status::InternalServerError)
        .map(|x| x.to_string())
}

#[rocket::get("/posts")]
fn article_posts(
    _user: AuthorizedUser,
    db: State<YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, rocket::http::Status> {
    let posts = db.list_posts().map_err(|_| Status::InternalServerError)?;
    let json_posts = serde_json::to_string(&posts).map_err(|_| Status::InternalServerError)?;
    Ok(rocket::response::content::Json(json_posts))
}

#[rocket::get("/post/<id>")]
fn single_post(
    id: usize,
    _user: AuthorizedUser,
    db: State<YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, rocket::http::Status> {
    let post = db.get_post(id).map_err(|_| Status::InternalServerError)?;
    let json_post = serde_json::to_string(&post).map_err(|_| Status::InternalServerError)?;
    Ok(rocket::response::content::Json(json_post))
}

fn main() {
    let db = db::YubanDatabase::new().expect("Could not open database");
    match db.new_login("me", "secret") {
        Ok(_) => {}
        Err(err) => {
            dbg!(err);
        }
    }
    rocket::ignite()
        .manage(db)
        .mount(
            "/",
            rocket::routes![login_post, article_posts, test_token, single_post, new_post],
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
