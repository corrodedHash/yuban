use crate::db;
use db::transactional;
use mysql::{
    params,
    prelude::{FromRow, Queryable},
};
use rocket::{http::Status, request::FromRequest, Route, State};

use super::InternalDebugFailure;

struct AuthorizedAdmin {
    pub username: String,
}
impl<'a, 'r> FromRequest<'a, 'r> for AuthorizedAdmin {
    type Error = ();

    fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let x = crate::auth::AuthorizedUser::from_request(request);
        let user = match x {
            rocket::Outcome::Success(x) => x,
            rocket::Outcome::Failure(f) => return rocket::Outcome::Failure(f),
            rocket::Outcome::Forward(_) => return rocket::Outcome::Forward(()),
        };
        if user.username == "admin" {
            rocket::Outcome::Success(Self {
                username: user.username,
            })
        } else {
            rocket::Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

#[rocket::get("/users")]
fn list_users(
    _user: AuthorizedAdmin,
    db: State<db::YubanDatabase>,
) -> rocket_contrib::json::Json<Vec<String>> {
    rocket_contrib::json::Json(db.list_users().ok().unwrap_or_default())
}

#[derive(serde::Deserialize)]
struct Username {
    pub username: String,
}

#[rocket::delete("/login", data = "<data>")]
fn remove_user(
    _user: AuthorizedAdmin,
    data: rocket_contrib::json::Json<Username>,
    db: State<db::YubanDatabase>,
) -> rocket::response::content::Json<String> {
    rocket::response::content::Json(
        db.remove_login(&data.username)
            .map(|_| "true")
            .ok()
            .unwrap_or("false")
            .to_owned(),
    )
}

#[derive(rocket::FromForm, serde::Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[rocket::put("/login", data = "<data>")]
fn add_user(
    _user: AuthorizedAdmin,
    data: rocket_contrib::json::Json<LoginData>,
    db: State<db::YubanDatabase>,
) -> rocket::response::content::Json<String> {
    rocket::response::content::Json(
        db.new_login(&data.username, &data.password)
            .map(|_| "true")
            .ok()
            .unwrap_or("false")
            .to_owned(),
    )
}

#[derive(Debug, serde::Serialize)]
struct GroupSummary {
    groupid: u64,
    groupname: String,
    users: Option<String>,
}

impl FromRow for GroupSummary {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (groupid, groupname, users): (u64, String, Option<String>) =
            mysql::prelude::FromRow::from_row_opt(row)?;
        Ok(Self {
            groupid,
            groupname,
            users,
        })
    }
}

#[rocket::get("/groups")]
fn summarize_groups(
    _user: AuthorizedAdmin,
    db: State<db::YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, InternalDebugFailure> {
    const STATEMENT_STRING: &str = crate::load_query!("summarize_groups.sql");
    let mut conn = db.get_conn()?;
    let statement = conn.prep(STATEMENT_STRING)?;
    let result: Vec<GroupSummary> = conn.exec(statement, ())?;
    let json_result = serde_json::to_string(&result)?;
    Ok(rocket::response::content::Json(json_result))
}

#[derive(serde::Deserialize)]
struct GroupName(String);

#[rocket::put("/group", data = "<data>")]
fn add_group(
    _user: AuthorizedAdmin,
    db: State<db::YubanDatabase>,
    data: rocket_contrib::json::Json<GroupName>,
) -> Result<rocket::response::content::Json<String>, InternalDebugFailure> {
    const STATEMENT_STRING: &str = concat!("INSERT INTO Groups (groupname) VALUES (:groupname)");
    let mut conn = db.get_conn()?;
    let mut transaction = conn.start_transaction(mysql::TxOpts::default())?;
    let statement = transaction.prep(STATEMENT_STRING)?;
    transaction.exec_drop(statement, params! {"groupname" => &data.0.0})?;
    let group_id = transaction.last_insert_id().unwrap();
    transaction.commit()?;
    Ok(rocket::response::content::Json(group_id.to_string()))
}

#[rocket::delete("/group", data = "<data>")]
fn remove_group(
    _user: AuthorizedAdmin,
    db: State<db::YubanDatabase>,
    data: rocket_contrib::json::Json<GroupName>,
) -> Result<Status, InternalDebugFailure> {
    const STATEMENT_STRING: &str = concat!("DELETE FROM Groups WHERE groupname = :groupname");
    let mut conn = db.get_conn()?;
    let statement = conn.prep(STATEMENT_STRING)?;
    conn.exec_drop(statement, params! {"groupname" => &data.0.0})?;

    Ok(Status::Ok)
}

#[derive(serde::Deserialize)]
struct GroupUser {
    username: String,
    groupname: String,
}

#[rocket::put("/group_user", data = "<data>")]
fn add_group_user(
    _user: AuthorizedAdmin,
    db: State<db::YubanDatabase>,
    data: rocket_contrib::json::Json<GroupUser>,
) -> Result<Status, InternalDebugFailure> {
    const STATEMENT_STRING: &str =
        concat!("INSERT INTO GroupMembership (groupid, userid) VALUES (:groupid, :userid)");
    let mut conn = db.get_conn()?;
    let mut transaction = conn.start_transaction(mysql::TxOpts::default())?;
    let userid = transactional::get_user_id(&data.username, &mut transaction)
        .map_err(|_| Status::BadRequest)?;
    let groupid = transactional::get_group_id(&data.groupname, &mut transaction)
        .map_err(|_| Status::BadRequest)?;
    let statement = transaction.prep(STATEMENT_STRING)?;
    transaction.exec_drop(
        statement,
        params! {"groupid" => groupid, "userid" => userid},
    )?;
    transaction.commit()?;
    Ok(Status::Ok)
}

#[rocket::delete("/group_user", data = "<data>")]
fn remove_group_user(
    _user: AuthorizedAdmin,
    db: State<db::YubanDatabase>,
    data: rocket_contrib::json::Json<GroupUser>,
) -> Result<Status, InternalDebugFailure> {
    const STATEMENT_STRING: &str =
        concat!("DELETE FROM GroupMembership WHERE userid = :userid AND groupid = :groupid");
    let mut conn = db.get_conn()?;
    let mut transaction = conn.start_transaction(mysql::TxOpts::default())?;
    let userid = transactional::get_user_id(&data.username, &mut transaction)?;
    let groupid = transactional::get_group_id(&data.groupname, &mut transaction)?;
    let statement = transaction.prep(STATEMENT_STRING)?;
    transaction.exec_drop(
        statement,
        params! {"groupid" => groupid, "userid" => userid},
    )?;
    transaction.commit()?;
    Ok(Status::Ok)
}

pub fn get_admin_routes() -> impl Into<Vec<Route>> {
    rocket::routes![
        add_user,
        remove_user,
        list_users,
        summarize_groups,
        add_group,
        remove_group,
        add_group_user,
        remove_group_user,
    ]
}
