use std::convert::{TryFrom, TryInto};

use crate::db::YubanDatabase;
use rand::Rng;
use rocket::{http::Status, request::FromRequest, State};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct AccessToken([u8; 32]);

impl TryFrom<Vec<u8>> for AccessToken {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let x = value.try_into().map_err(|_| ())?;
        Ok(Self(x))
    }
}
impl Default for AccessToken {
    fn default() -> Self {
        let mut token = [0_u8; 32];
        rand::thread_rng().fill(&mut token[..]);
        Self(token)
    }
}
impl From<AccessToken> for Vec<u8> {
    fn from(x: AccessToken) -> Self {
        x.0.to_vec()
    }
}
impl AsRef<[u8; 32]> for AccessToken {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Debug)]
pub struct AuthorizedUser {
    pub userid: u64,
    pub username: String,
    pub token: AccessToken,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthorizedUser {
    type Error = ();

    fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        fn gather_info(request: &'_ rocket::Request<'_>) -> Result<AuthorizedUser, Status> {
            let db = match request.guard::<State<YubanDatabase>>() {
                rocket::Outcome::Success(db) => db,
                rocket::Outcome::Failure(_) | rocket::Outcome::Forward(_) => {
                    return Err(Status::InternalServerError);
                }
            };
            let cookies = request.cookies();
            let token_cookie = cookies.get("token").ok_or(Status::Unauthorized)?.value();
            let (username, token): (&str, &str) = (
                &token_cookie[..token_cookie.len() - 44],
                &token_cookie[token_cookie.len() - 44..],
            );

            let decoded_token_vec =
                base64::decode_config(token, base64::URL_SAFE).map_err(|err| {
                    dbg!(err);
                    Status::BadRequest
                })?;
            let decoded_token = decoded_token_vec
                .try_into()
                .map_err(|_| rocket::http::Status::BadRequest)?;

            let token_check = db.check_token(username, &decoded_token);
            if let Ok(userid) = token_check {
                Ok(AuthorizedUser {
                    userid,
                    username: username.to_owned(),
                    token: decoded_token,
                })
            } else {
                Err(Status::Unauthorized)
            }
        }
        match gather_info(request) {
            Ok(x) => rocket::request::Outcome::Success(x),
            Err(status) => rocket::request::Outcome::Failure((status, ())),
        }
    }
}
