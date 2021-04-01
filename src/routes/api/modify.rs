use std::io::Read;

use rocket::{http::Status, State};

use crate::{auth::AuthorizedUser, db};

use super::LangCode;

pub(super) struct BoundedPost {
    post: String,
}

impl rocket::data::FromDataSimple for BoundedPost {
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
pub(super) fn new_post(
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
pub(super) fn new_translation(
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
pub(super) fn new_correction(
    user: AuthorizedUser,
    data: BoundedPost,
    origpostid: u64,
    db: State<db::YubanDatabase>,
) -> Result<String, Status> {
    db.add_correction(&user.username, &data.post, origpostid)
        .map_err(|_| Status::InternalServerError)
        .map(|x| x.to_string())
}
