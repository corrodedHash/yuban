use std::io::Read;

use crate::db::YubanDatabase;
use crate::{auth::AuthorizedUser, db};
use rocket::{
    data::FromDataSimple,
    http::{Cookie, Cookies, Status},
    request::FromParam,
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
pub struct LangCode(pub String);

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

struct BoundedPost {
    post: String,
}

impl FromDataSimple for BoundedPost {
    type Error = ();

    fn from_data(
        _request: &rocket::Request,
        data: rocket::Data,
    ) -> rocket::data::Outcome<Self, Self::Error> {
        let mut buffer = String::new();
        if data.open().take(65536).read_to_string(&mut buffer).is_err() {
            return rocket::Outcome::Failure((Status::BadRequest, ()));
        }
        if buffer.len() >= 65536 {
            return rocket::Outcome::Failure((Status::BadRequest, ()));
        }
        rocket::Outcome::Success(BoundedPost { post: buffer })
    }
}

#[rocket::put("/newpost/<langcode>", data = "<data>")]
fn new_post(
    user: AuthorizedUser,
    langcode: LangCode,
    data: BoundedPost,
    db: State<db::YubanDatabase>,
) -> Result<String, Status> {
    db.add_new_thread(&user.username, &data.post, &langcode.0)
        .map_err(|_| Status::InternalServerError)
        .map(|(thread_id, post_id)| format!("{{'thread_id':{}, 'post_id':{}}}", thread_id, post_id))
}

#[rocket::put("/addpost/<thread_id>/<langcode>", data = "<data>")]
fn new_translation(
    user: AuthorizedUser,
    data: BoundedPost,
    thread_id: u64,
    langcode: LangCode,
    db: State<db::YubanDatabase>,
) -> Result<String, Status> {
    db.add_post(&user.username, &data.post, thread_id, &langcode.0)
        .map_err(|_| Status::InternalServerError)
        .map(|x| x.to_string())
}

#[rocket::put("/addcorrection/<origpostid>", data = "<data>")]
fn new_correction(
    user: AuthorizedUser,
    data: BoundedPost,
    origpostid: u64,
    db: State<db::YubanDatabase>,
) -> Result<String, Status> {
    db.add_correction(&user.username, &data.post, origpostid)
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
pub fn get_api_routes() -> impl Into<Vec<rocket::Route>> {
    rocket::routes![
        login_post,
        article_posts,
        test_token,
        single_post,
        new_post,
        new_translation,
        new_correction
    ]
}
