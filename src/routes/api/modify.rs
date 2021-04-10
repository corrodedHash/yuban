use std::io::Read;

use mysql::{params, prelude::Queryable};
use rocket::{http::Status, State};

use crate::{auth::AuthorizedUser, db};

use super::{super::InternalDebugFailure, LangCode};

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

#[rocket::put("/addthread/<group_id>/<langcode>", data = "<data>")]
pub(super) fn new_thread(
    user: AuthorizedUser,
    langcode: LangCode,
    data: BoundedPost,
    group_id: u64,
    db: State<db::YubanDatabase>,
) -> Result<String, InternalDebugFailure> {
    const STATEMENT_STRING: &str = concat!(
        "INSERT INTO Threads (creator, groupid, opened_on) ",
        "VALUES (:owner_id, :group_id, CURRENT_TIMESTAMP) "
    );
    let mut conn = db.get_conn()?;

    let user_permission = crate::db::permissions::add_thread(user.userid, group_id, &mut conn)?;
    if !user_permission {
        return Err(Status::Unauthorized.into());
    }

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
    }))?)
}

#[rocket::put("/addpost/<thread_id>/<langcode>", data = "<data>")]
pub(super) fn new_translation(
    user: AuthorizedUser,
    data: BoundedPost,
    thread_id: u64,
    langcode: LangCode,
    db: State<db::YubanDatabase>,
) -> Result<String, InternalDebugFailure> {
    let mut conn = db.get_conn()?;

    let user_permission =
        crate::db::permissions::add_translation(user.userid, thread_id, &mut conn)?;
    if !user_permission {
        return Err(Status::Unauthorized.into());
    }

    let mut transaction = conn
        .start_transaction(mysql::TxOpts::default())
        .map_err(|err| {
            dbg!(err);
        })?;
    let post_id = crate::db::transactional::post(user.userid, &data.post, &mut transaction)?;
    crate::db::transactional::link_orig(thread_id, post_id, &langcode.0, &mut transaction)
        .map(|_| post_id)?;

    transaction.commit()?;
    Ok(post_id.to_string())
}

#[rocket::put("/addcorrection/<origpostid>", data = "<data>")]
pub(super) fn new_correction(
    user: AuthorizedUser,
    data: BoundedPost,
    origpostid: u64,
    db: State<db::YubanDatabase>,
) -> Result<String, InternalDebugFailure> {
    const STATEMENT_STRING_ORIG_LINK: &str = concat!(
        "INSERT INTO Corrections (orig_id, post_id) ",
        "VALUES (:orig_id, :post_id)"
    );
    let mut conn = db.get_conn()?;

    let user_permission =
        crate::db::permissions::add_correction(user.userid, origpostid, &mut conn)?;
    if !user_permission {
        return Err(Status::Unauthorized.into());
    }

    let mut transaction = conn.start_transaction(mysql::TxOpts::default())?;
    let postid = crate::db::transactional::post(user.userid, &data.post, &mut transaction)?;

    let statement = transaction.prep(STATEMENT_STRING_ORIG_LINK)?;
    let params = params! {"orig_id" => origpostid, "post_id" => postid};
    transaction.exec_drop(statement, params)?;
    transaction.commit()?;
    Ok(postid.to_string())
}
