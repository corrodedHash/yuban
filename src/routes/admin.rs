use crate::db;
use mysql::prelude::{FromRow, Queryable};
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
    groupname: String,
    users: String,
}

impl FromRow for GroupSummary {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (groupname, users): (String, String) = mysql::prelude::FromRow::from_row_opt(row)?;
        Ok(Self { groupname, users })
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

    Ok(rocket::response::content::Json(serde_json::to_string(
        &result,
    )?))
}

pub fn get_admin_routes() -> impl Into<Vec<Route>> {
    rocket::routes![add_user, remove_user, list_users, summarize_groups]
}
