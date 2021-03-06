use mysql::{params, prelude::Queryable};

use super::QueryableAndLastInsert;

pub(crate) fn get_user_id<Q: Queryable>(username: &str, transaction: &mut Q) -> Result<u64, ()> {
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

pub(crate) fn get_group_id<Q: Queryable>(groupname: &str, transaction: &mut Q) -> Result<u64, ()> {
    const STATEMENT_STRING_ID: &str = "SELECT id FROM Groups WHERE groupname = :groupname";

    let statement = transaction.prep(STATEMENT_STRING_ID).map_err(|err| {
        dbg!(err);
    })?;
    let params = params! {"groupname" => groupname};
    transaction
        .exec_first(statement, params)
        .map_err(|err| {
            dbg!(err);
        })?
        .ok_or(())
}

pub(crate) fn post<Q: QueryableAndLastInsert>(
    user_id: u64,
    post: &str,
    transaction: &mut Q,
) -> Result<u64, ()> {
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
    let postid = transaction.last_insert_id();
    Ok(postid)
}

pub(crate) fn link_orig<Q: Queryable>(
    thread_id: u64,
    post_id: u64,
    langcode: &str,
    transaction: &mut Q,
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
