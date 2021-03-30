use crate::db;
use rocket::{http::Status, request::FromRequest, Route, State};

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

#[rocket::delete("/login", data = "<data>")]
fn remove_user(
    _user: AuthorizedAdmin,
    data: rocket_contrib::json::Json<String>,
    db: State<db::YubanDatabase>,
) -> rocket::response::content::Json<String> {
    rocket::response::content::Json(
        db.remove_login(&data)
            .map(|_| "true")
            .ok()
            .unwrap_or("false")
            .to_owned(),
    )
}

pub fn get_admin_routes() -> impl Into<Vec<Route>> {
    rocket::routes![add_user, remove_user]
}
