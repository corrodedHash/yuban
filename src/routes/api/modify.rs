use std::io::Read;

use mysql::{params, prelude::Queryable};
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

#[derive(Debug, Clone)]
pub(super) struct InternalDebugFailure(Status);

impl From<Status> for InternalDebugFailure {
    fn from(s: Status) -> Self {
        Self(s)
    }
}

impl From<()> for InternalDebugFailure {
    fn from(_: ()) -> Self {
        Self(Status::InternalServerError)
    }
}
impl From<mysql::Error> for InternalDebugFailure {
    fn from(err: mysql::Error) -> Self {
        dbg!(err);
        Self(Status::InternalServerError)
    }
}

impl<'r> rocket::response::Responder<'r> for InternalDebugFailure {
    fn respond_to(self, request: &rocket::Request) -> rocket::response::Result<'r> {
        self.0.respond_to(request)
    }
}

#[rocket::put("/newpost/<group_id>/<langcode>", data = "<data>")]
pub(super) fn new_thread(
    user: AuthorizedUser,
    langcode: LangCode,
    data: BoundedPost,
    group_id: u64,
    db: State<db::YubanDatabase>,
) -> Result<String, InternalDebugFailure> {
    const STATEMENT_STRING: &str = concat!(
        "INSERT INTO Threads (creator, group, opened_on) ",
        "VALUES (:owner_id, :group_id, CURRENT_TIMESTAMP) "
    );
    let mut conn = db.get_conn()?;
    let mut transaction = conn.start_transaction(mysql::TxOpts::default())?;
    let user_id = crate::db::transactional::get_user_id(&user.username, &mut transaction)?;

    let statement = transaction.prep(STATEMENT_STRING)?;
    let params = params! {"owner_id" => user_id, "group_id" => group_id};
    transaction.exec_drop(statement, params)?;
    let thread_id = transaction.last_insert_id().ok_or(())?;
    let post_id = crate::db::transactional::post(user_id, &data.post, &mut transaction)?;
    crate::db::transactional::link_orig(thread_id, post_id, &langcode.0, &mut transaction)
        .map(|_| post_id)?;

    transaction.commit()?;
    Ok(serde_json::to_string(&serde_json::json!({
        "thread_id": thread_id,
        "post_id": post_id
    }))
    .map_err(|err| {
        dbg!(err);
        Status::BadGateway
    })?)
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
