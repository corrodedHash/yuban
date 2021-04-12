use mysql::{params, prelude::Queryable};
use rocket::{http::Status, State};

use crate::{
    auth::AuthorizedUser,
    db::{SkyDate, YubanDatabase},
    load_query,
};

use super::LangCode;
use crate::routes::InternalDebugFailure;

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
pub(super) fn list_groups(
    user: AuthorizedUser,
    db: State<YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, InternalDebugFailure> {
    let mut conn = db.get_conn()?;
    let statement = conn.prep(load_query!("list_groups.sql"))?;
    let group: Vec<GroupSummary> = conn.exec(statement, params! {"username" => user.username})?;
    let json_post = serde_json::to_string(&group)?;
    Ok(rocket::response::content::Json(json_post))
}

#[derive(Debug, Clone, serde::Serialize)]
struct ThreadSummary {
    id: u64,
    creator: String,
    opened_on: crate::db::SkyDate,
    languages: String,
}

impl mysql::prelude::FromRow for ThreadSummary {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (id, creator, opened_on, languages) = mysql::prelude::FromRow::from_row_opt(row)?;
        Ok(Self {
            id,
            creator,
            opened_on,
            languages,
        })
    }
}

#[rocket::get("/summary_group/<group_id>")]
pub(super) fn list_threads(
    user: AuthorizedUser,
    group_id: u64,
    db: State<YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, InternalDebugFailure> {
    let mut conn = db.get_conn()?;

    let user_permission = crate::db::permissions::view_group(user.userid, group_id, &mut conn)?;
    if !user_permission {
        return Err(Status::Unauthorized.into());
    }
    let statement = conn.prep(load_query!("list_threads.sql"))?;
    let group: Vec<ThreadSummary> = conn.exec(statement, params! {"groupid" => group_id})?;
    let json_post = serde_json::to_string(&group)?;
    Ok(rocket::response::content::Json(json_post))
}

#[derive(Debug, Clone, serde::Serialize)]
struct PostSummary {
    id: u64,
    opened_on: SkyDate,
    ellipsis: String,
    username: String,
    lang: String,
    corrections: String,
}

impl mysql::prelude::FromRow for PostSummary {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (id, opened_on, ellipsis, username, lang, corrections) =
            mysql::prelude::FromRow::from_row_opt(row)?;
        Ok(Self {
            id,
            opened_on,
            ellipsis,
            username,
            lang,
            corrections,
        })
    }
}

#[rocket::get("/summary_thread/<thread_id>")]
pub(super) fn list_posts(
    user: AuthorizedUser,
    thread_id: u64,
    db: State<YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, InternalDebugFailure> {
    let mut conn = db.get_conn()?;

    let user_permission = crate::db::permissions::view_thread(user.userid, thread_id, &mut conn)?;
    if !user_permission {
        return Err(Status::Unauthorized.into());
    }

    let statement = conn.prep(load_query!("list_posts.sql"))?;
    let group: Vec<PostSummary> = conn.exec(statement, params! {"thread_id" => thread_id})?;
    let json_post = serde_json::to_string(&group)?;
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

#[rocket::get("/post/<post_id>")]
pub(super) fn single_post(
    post_id: u64,
    user: AuthorizedUser,
    db: State<YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, InternalDebugFailure> {
    let mut conn = db.get_conn()?;

    let user_permission = crate::db::permissions::view_post(user.userid, post_id, &mut conn)?;
    if !user_permission {
        return Err(Status::Unauthorized.into());
    }

    let statement = conn.prep(load_query!("get_post.sql"))?;
    let post: Post = conn
        .exec_first(statement, params! {"postid" => post_id})?
        .ok_or(())?;
    let json_post = serde_json::to_string(&post)?;
    Ok(rocket::response::content::Json(json_post))
}
