use rocket::{State, http::{Cookie, Cookies, Status}, request::FromRequest};

use crate::{auth::AuthorizedUser, db};

#[derive(rocket::FromForm, serde::Deserialize)]
pub(super) struct LoginData {
    username: String,
    password: String,
}

#[rocket::post("/login", data = "<data>")]
pub(super) fn login_post(
    data: rocket_contrib::json::Json<LoginData>,
    mut cookies: Cookies,
    db: State<db::YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, Status> {
    if let Ok(user_id) = db.check_login(&data.username, &data.password) {
        let token = match db.add_token(user_id) {
            Ok(t) => t,
            Err(_) => return Err(Status::BadGateway),
        };
        let str_token = base64::encode_config(token.as_ref(), base64::URL_SAFE);
        assert_eq!(str_token.len(), 44);
        cookies.add(
            Cookie::build("token", format!("{}{}", data.username, str_token))
                .secure(true)
                .same_site(rocket::http::SameSite::Strict)
                .permanent()
                .path("/")
                .http_only(true)
                .finish(),
        );
        return Ok(rocket::response::content::Json("true".to_owned()));
    }
    Ok(rocket::response::content::Json("false".to_owned()))
}

#[rocket::post("/logout")]
pub fn logout_post(
    user: AuthorizedUser,
    mut cookies: Cookies,
    db: State<db::YubanDatabase>,
) -> Result<rocket::response::content::Json<String>, Status> {
    db.remove_token(user.userid, user.token)
        .map_err(|_| Status::BadGateway)?;
    cookies.remove(Cookie::build("token", "").path("/").finish());
    Ok(rocket::response::content::Json("true".to_owned()))
}

#[rocket::get("/testtoken")]
pub fn test_token(user: Result<AuthorizedUser, <AuthorizedUser as FromRequest>::Error>) -> Result<rocket::response::content::Json<String>, Status> {
    match user {
        Ok(user) => {    Ok(rocket::response::content::Json(format!("{{\"username\": \"{}\"}}", user.username)))
    }
        Err(_) => {
    Ok(rocket::response::content::Json("null".to_owned()))

        }
    }
}
