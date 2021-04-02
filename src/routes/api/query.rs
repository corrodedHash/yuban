use mysql::{params, prelude::Queryable};
use rocket::{http::Status, State};

use crate::{
    auth::AuthorizedUser,
    db::{SkyDate, YubanDatabase},
    load_query,
};

use super::LangCode;

#[derive(Debug, Clone, serde::Serialize)]
struct GroupSummary {
    groupid: u64,
    groupname: String,
    threadcount: u64,
}

impl mysql::prelude::FromRow for GroupSummary {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (groupid, groupname, threadcount) = mysql::prelude::FromRow::from_row_opt(row)?;
        Ok(Self {
            groupid,
            groupname,
            threadcount,
        })
    }
}

#[rocket::get("/summary")]
pub fn list_groups(
    user: AuthorizedUser,
    db: State<YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, Status> {
    let mut conn = db.get_conn().map_err(|err| {
        dbg!(err);
        Status::BadGateway
    })?;
    let statement = conn.prep(load_query!("list_groups.sql")).map_err(|err| {
        dbg!(err);
        Status::BadGateway
    })?;
    let group: GroupSummary = conn
        .exec_first(statement, params! {"username" => user.username})
        .map_err(|err| {
            dbg!(err);
        })
        .and_then(|p| p.ok_or(()))
        .map_err(|_| Status::InternalServerError)?;
    let json_post = serde_json::to_string(&group).map_err(|_| Status::InternalServerError)?;
    Ok(rocket::response::content::Json(json_post))
}

#[derive(Debug, Clone, serde::Serialize)]
struct ThreadSummary {
    id: u64,
    creator: String,
    opened_on: crate::db::SkyDate,
    langcodes: String,
    correction_counts: String,
}

impl mysql::prelude::FromRow for ThreadSummary {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (id, creator, opened_on, langcodes, correction_counts) =
            mysql::prelude::FromRow::from_row_opt(row)?;
        Ok(Self {
            id,
            creator,
            opened_on,
            langcodes,
            correction_counts,
        })
    }
}

#[rocket::get("/summary_group/<group_id>")]
pub fn list_threads(
    _user: AuthorizedUser,
    group_id: u64,
    db: State<YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, Status> {
    let mut conn = db.get_conn().map_err(|err| {
        dbg!(err);
        Status::BadGateway
    })?;
    let statement = conn.prep(load_query!("list_threads.sql")).map_err(|err| {
        dbg!(err);
        Status::BadGateway
    })?;
    let group: ThreadSummary = conn
        .exec_first(statement, params! {"groupid" => group_id})
        .map_err(|err| {
            dbg!(err);
        })
        .and_then(|p| p.ok_or(()))
        .map_err(|_| Status::InternalServerError)?;
    let json_post = serde_json::to_string(&group).map_err(|_| Status::InternalServerError)?;
    Ok(rocket::response::content::Json(json_post))
}

#[derive(Debug, Clone, serde::Serialize)]
struct PostSummary {
    id: u64,
    opened_on: SkyDate,
    user: String,
    post: String,
}

impl mysql::prelude::FromRow for PostSummary {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (id, opened_on, user, post) = mysql::prelude::FromRow::from_row_opt(row)?;
        Ok(Self {
            id,
            opened_on,
            user,
            post,
        })
    }
}

#[rocket::get("/summary_thread/<thread_id>")]
pub fn list_posts(
    _user: AuthorizedUser,
    thread_id: u64,
    db: State<YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, Status> {
    let mut conn = db.get_conn().map_err(|err| {
        dbg!(err);
        Status::BadGateway
    })?;
    let statement = conn.prep(load_query!("list_posts.sql")).map_err(|err| {
        dbg!(err);
        Status::BadGateway
    })?;
    let group: PostSummary = conn
        .exec_first(statement, params! {"thread_id" => thread_id})
        .map_err(|err| {
            dbg!(err);
        })
        .and_then(|p| p.ok_or(()))
        .map_err(|_| Status::InternalServerError)?;
    let json_post = serde_json::to_string(&group).map_err(|_| Status::InternalServerError)?;
    Ok(rocket::response::content::Json(json_post))
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Post {
    pub thread_id: u64,
    pub id: u64,
    pub posttime: SkyDate,
    pub user: String,
    pub langcode: LangCode,
    pub correction_for: Option<u64>,
    pub text: String,
}

impl mysql::prelude::FromRow for Post {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (thread_id, id, posttime, user, langcode, correction_for, text) =
            mysql::prelude::FromRow::from_row_opt(row)?;
        Ok(Self {
            thread_id,
            id,
            posttime,
            user,
            langcode: LangCode(langcode),
            correction_for,
            text,
        })
    }
}

#[rocket::get("/post/<id>")]
pub fn single_post(
    id: usize,
    _user: AuthorizedUser,
    db: State<YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, Status> {
    let mut conn = db.get_conn().map_err(|err| {
        dbg!(err);
        Status::BadGateway
    })?;
    let statement = conn.prep(load_query!("get_post.sql")).map_err(|err| {
        dbg!(err);
        Status::BadGateway
    })?;
    let post: Post = conn
        .exec_first(statement, params! {"postid" => id})
        .map_err(|err| {
            dbg!(err);
        })
        .and_then(|p| p.ok_or(()))
        .map_err(|_| Status::InternalServerError)?;
    let json_post = serde_json::to_string(&post).map_err(|_| Status::InternalServerError)?;
    Ok(rocket::response::content::Json(json_post))
}
