
use mysql::{params, prelude::Queryable, Transaction};

pub(super) fn get_user_id(username: &str, transaction: &mut Transaction) -> Result<u64, ()> {
    const STATEMENT_STRING_ID: &str = "SELECT id FROM Users WHERE username = :username";

    let statement = transaction.prep(STATEMENT_STRING_ID).map_err(|err| {
        dbg!(err);
    })?;
    let params = params! {"username" => username};
    transaction
        .exec_first(statement, params)
        .map_err(|err| {
            dbg!(err);
        })?
        .ok_or(())
}
pub(super) fn post(user_id: u64, post: &str, transaction: &mut Transaction) -> Result<u64, ()> {
    const STATEMENT_STRING_INSERT: &str = concat!(
        "INSERT INTO Posts (userid, postdate, post) ",
        "VALUES (:userid, CURRENT_TIMESTAMP, :post)",
    );
    let statement = transaction.prep(STATEMENT_STRING_INSERT).map_err(|err| {
        dbg!(err);
    })?;
    let params = params! {"userid" => user_id, "post" => post};
    transaction.exec_drop(statement, params).map_err(|err| {
        dbg!(err);
    })?;
    let postid = transaction.last_insert_id().ok_or(())?;
    Ok(postid)
}

pub(super) fn link_orig(
    thread_id: u64,
    post_id: u64,
    langcode: &str,
    transaction: &mut Transaction,
) -> Result<(), ()> {
    const STATEMENT_STRING_ORIG_LINK: &str = concat!(
        "INSERT INTO Originals (thread_id, post_id, langcode) ",
        "VALUES (:thread_id, :post_id, :langcode)"
    );

    let statement = transaction
        .prep(STATEMENT_STRING_ORIG_LINK)
        .map_err(|err| {
            dbg!(err);
        })?;
    let params = params! {"thread_id" => thread_id, "post_id" => post_id, "langcode" => langcode};
    transaction.exec_drop(statement, params).map_err(|err| {
        dbg!(err);
    })?;
    Ok(())
}
