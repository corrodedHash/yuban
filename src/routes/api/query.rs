use mysql::params;
use rocket::{http::Status, State};

use crate::{auth::AuthorizedUser, db::YubanDatabase};

#[rocket::get("/summary")]
pub fn list_groups(
    _user: AuthorizedUser,
    db: State<YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, Status> {
    let posts = db
        .list_original_posts()
        .map_err(|_| Status::InternalServerError)?;
    let json_posts = serde_json::to_string(&posts).map_err(|_| Status::InternalServerError)?;
    Ok(rocket::response::content::Json(json_posts))
}

#[rocket::get("/summary_group/<group_id>")]
pub fn list_threads(
    _user: AuthorizedUser,
    group_id: u64,
    db: State<YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, Status> {
    let posts = db
        .list_original_posts()
        .map_err(|_| Status::InternalServerError)?;
    let json_posts = serde_json::to_string(&posts).map_err(|_| Status::InternalServerError)?;
    Ok(rocket::response::content::Json(json_posts))
}

#[rocket::get("/summary_thread/<group_id>/<thread_id>")]
pub fn list_posts(
    _user: AuthorizedUser,
    group_id: u64,
    thread_id: u64,
    db: State<YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, Status> {
    let posts = db
        .list_original_posts()
        .map_err(|_| Status::InternalServerError)?;
    let json_posts = serde_json::to_string(&posts).map_err(|_| Status::InternalServerError)?;
    Ok(rocket::response::content::Json(json_posts))
}

use crate::db::load_query;

#[rocket::get("/post/<id>")]
pub fn single_post(
    id: usize,
    _user: AuthorizedUser,
    db: State<YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, Status> {
    const STATEMENT_STRING: &str = crate::db::load_query!("get_post.sql");
    let mut conn = self.get_conn()?;
    let statement = conn.prep(STATEMENT_STRING).map_err(|err| {
        dbg!(err);
    })?;
    conn.exec_first(statement, params! {"postid" => postid})
        .map_err(|err| {
            dbg!(err);
        })
        .and_then(|p| p.ok_or(()))
        .map_err(|_| Status::InternalServerError)?;
    let post = db.get_post(id).map_err(|_| Status::InternalServerError)?;
    let json_post = serde_json::to_string(&post).map_err(|_| Status::InternalServerError)?;
    Ok(rocket::response::content::Json(json_post))
}
