use mysql::{params, prelude::Queryable, Params};

fn row_exists<Q: Queryable, P: Into<Params>>(
    query: &str,
    params: P,
    transaction: &mut Q,
) -> Result<bool, ()> {
    let statement = transaction.prep(query).map_err(|err| {
        dbg!(err);
    })?;

    let count: Option<u64> = transaction.exec_first(statement, params).map_err(|err| {
        dbg!(err);
    })?;
    Ok(count.is_some())
}

pub fn view_group<Q: Queryable>(
    user_id: u64,
    group_id: u64,
    transaction: &mut Q,
) -> Result<bool, ()> {
    const STATEMENT_STRING: &str = concat!(
        "SELECT 1 FROM GroupMembership ",
        "WHERE userid = :user_id AND groupid = :group_id"
    );
    let p = params! {
        "user_id" => user_id,
        "group_id" => group_id,
    };
    row_exists(STATEMENT_STRING, p, transaction)
}

pub fn add_thread<Q: Queryable>(
    user_id: u64,
    group_id: u64,
    transaction: &mut Q,
) -> Result<bool, ()> {
    view_group(user_id, group_id, transaction)
}

pub fn view_thread<Q: Queryable>(
    user_id: u64,
    thread_id: u64,
    transaction: &mut Q,
) -> Result<bool, ()> {
    const STATEMENT_STRING: &str = concat!(
        "SELECT 1 FROM GroupMembership ",
        "JOIN Threads ON GroupMembership.groupid = Threads.groupid ",
        "WHERE GroupMembership.userid = :user_id AND Threads.id = :thread_id"
    );
    let p = params! {
        "user_id" => user_id,
        "thread_id" => thread_id,
    };
    row_exists(STATEMENT_STRING, p, transaction)
}

pub fn add_translation<Q: Queryable>(
    user_id: u64,
    thread_id: u64,
    transaction: &mut Q,
) -> Result<bool, ()> {
    view_thread(user_id, thread_id, transaction)
}

pub fn view_post<Q: Queryable>(
    user_id: u64,
    post_id: u64,
    transaction: &mut Q,
) -> Result<bool, ()> {
    const STATEMENT_STRING: &str = crate::load_query!("permission_view_post.sql");
    let p = params! {
        "user_id" => user_id,
        "post_id" => post_id,
    };
    row_exists(STATEMENT_STRING, p, transaction)
}

pub fn add_correction<Q: Queryable>(
    user_id: u64,
    post_id: u64,
    transaction: &mut Q,
) -> Result<bool, ()> {
    view_post(user_id, post_id, transaction)
}

pub fn view_correction<Q: Queryable>(
    user_id: u64,
    orig_postid: u64,
    transaction: &mut Q,
) -> Result<bool, ()> {
    const STATEMENT_STRING: &str = concat!(
        "SELECT 1 FROM GroupMembership ",
        "JOIN Threads ON GroupMembership.groupid = Threads.groupid ",
        "JOIN Originals ON Originals.thread_id = Threads.id ",
        "JOIN Corrections ON Corrections.orig_id = Originals.post_id ",
        "WHERE GroupMembership.userid = :userid AND Corrections.post_id = :postid ",
    );
    let p = params! {
        "userid" => user_id,
        "postid" => orig_postid,
    };
    row_exists(STATEMENT_STRING, p, transaction)
}
