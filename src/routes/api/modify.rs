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
) -> Result<rocket::response::content::Json<String>, InternalDebugFailure> {
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

    let statement = transaction.prep(STATEMENT_STRING)?;
    let params = params! {"owner_id" => user.userid, "group_id" => group_id};
    transaction.exec_drop(statement, params)?;
    let thread_id = transaction.last_insert_id().ok_or(())?;
    let post_id = crate::db::transactional::post(user.userid, &data.post, &mut transaction)?;
    crate::db::transactional::link_orig(thread_id, post_id, &langcode.0, &mut transaction)
        .map(|_| post_id)?;

    transaction.commit()?;
    Ok(rocket::response::content::Json(serde_json::to_string(
        &serde_json::json!({
            "thread_id": thread_id,
            "post_id": post_id
        }),
    )?))
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

#[allow(unused_variables, unreachable_code)]
#[rocket::delete("/thread/<thread_id>")]
pub(super) fn delete_thread(
    user: AuthorizedUser,
    thread_id: u64,
    db: State<db::YubanDatabase>,
) -> Result<rocket::response::content::Json<&str>, InternalDebugFailure> {
    return Err(Status::ServiceUnavailable.into());

    const STATEMENT_STRING: &str = concat!(
        "DELETE FROM Threads WHERE ",
        "Threads.id = :thread_id AND Threads.creator = :creator"
    );
    let mut conn = db.get_conn()?;

    let mut transaction = conn.start_transaction(mysql::TxOpts::default())?;

    let statement = transaction.prep(STATEMENT_STRING)?;
    let params = params! {"thread_id" => thread_id, "creator" => user.userid};
    transaction.exec_drop(statement, params)?;
    let deletion_worked = transaction.affected_rows() > 0;

    transaction.commit()?;
    Ok(rocket::response::content::Json(if deletion_worked {
        "true"
    } else {
        "false"
    }))
}

#[rocket::delete("/post/<post_id>")]
pub(super) fn delete_post(
    user: AuthorizedUser,
    post_id: u64,
    db: State<db::YubanDatabase>,
) -> Result<rocket::response::content::Json<&str>, InternalDebugFailure> {
    const STATEMENT_STRING: &str = concat!(
        "DELETE FROM Posts WHERE ",
        "id = :post_id AND userid = :userid"
    );
    let mut conn = db.get_conn()?;

    let mut transaction = conn.start_transaction(mysql::TxOpts::default())?;

    let statement = transaction.prep(STATEMENT_STRING)?;
    let params = params! {"post_id" => post_id, "userid" => user.userid};
    transaction.exec_drop(statement, params)?;
    let deletion_worked = transaction.affected_rows() > 0;

    transaction.commit()?;
    Ok(rocket::response::content::Json(if deletion_worked {
        "true"
    } else {
        "false"
    }))
}

#[rocket::delete("/correction/<post_id>")]
pub(super) fn delete_correction(
    user: AuthorizedUser,
    post_id: u64,
    db: State<db::YubanDatabase>,
) -> Result<rocket::response::content::Json<&str>, InternalDebugFailure> {
    delete_post(user, post_id, db)
}
