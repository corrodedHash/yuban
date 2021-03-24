use crate::{auth::AccessToken, LangCode};
use mysql::{
    params,
    prelude::{FromRow, Queryable},
    OptsBuilder,
};
use rand::RngCore;
use sha2::{digest::FixedOutput, Digest, Sha256};

#[derive(Clone, Debug, serde::Serialize)]
pub struct Post {
    pub thread_id: u64,
    pub id: u64,
    #[serde(serialize_with = "serialize_date")]
    pub posttime: mysql::Value,
    pub user: String,
    pub langcode: LangCode,
    pub correction_for: Option<u64>,
    pub text: String,
}

impl FromRow for Post {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (thread_id, id, posttime, user, langcode, correction_for, text) =
            mysql::prelude::FromRow::from_row_opt(row)?;
        Ok(Self {
            thread_id,
            id,
            posttime,
            user,
            langcode: LangCode(langcode),
            correction_for,
            text,
        })
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ThreadSummary {
    pub id: u64,
    #[serde(serialize_with = "serialize_date")]
    pub posttime: mysql::Value,
    pub user: String,
    pub posts: String,
}

impl FromRow for ThreadSummary {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (id, posttime, user, posts) = mysql::prelude::FromRow::from_row_opt(row)?;
        Ok(Self {
            id,
            posttime,
            user,
            posts,
        })
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct CorrectionPostSummary {
    pub id: u64,
    pub ellipse: String,
    pub author: String,
    #[serde(serialize_with = "serialize_date")]
    pub posttime: mysql::Value,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct TranslationPostSummary {
    pub id: u64,
    pub ellipse: String,
    pub author: String,
    pub langcode: String,
    #[serde(serialize_with = "serialize_date")]
    pub posttime: mysql::Value,
    pub corrections: Vec<CorrectionPostSummary>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct OriginalPostSummary {
    pub post: Post,
    pub translations: Vec<TranslationPostSummary>,
    pub corrections: Vec<CorrectionPostSummary>,
}

fn serialize_date<S: serde::Serializer>(x: &mysql::Value, s: S) -> Result<S::Ok, S::Error> {
    use serde::ser::Error;
    if let mysql::Value::Date(year, month, day, hour, minutes, seconds, ms) = x {
        s.serialize_str(&format!(
            "{}-{}-{} {}:{}:{}.{}",
            year, month, day, hour, minutes, seconds, ms
        ))
    } else if let mysql::Value::Bytes(x) = x {
        s.serialize_str(
            &std::str::from_utf8(x).map_err(|_| S::Error::custom("UTF-8 error in date"))?,
        )
    } else {
        dbg!(x);
        Err(S::Error::custom("Value not a date"))
    }
}

fn salted_pw(pass: &str, salt: &[u8]) -> [u8; 32] {
    let mut h = Sha256::default();
    h.update(pass.as_bytes());
    h.update(salt);
    h.finalize_fixed().into()
}

fn compare_slice<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).all(|(x, y)| x == y)
}

fn generate_salt() -> Vec<u8> {
    let mut salt_vec = [0_u8; 16];
    rand::thread_rng().fill_bytes(&mut salt_vec);
    salt_vec.to_vec()
}

mod transactional_yuban {
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
        let params =
            params! {"thread_id" => thread_id, "post_id" => post_id, "langcode" => langcode};
        transaction.exec_drop(statement, params).map_err(|err| {
            dbg!(err);
        })?;
        Ok(())
    }
}

pub struct YubanDatabase {
    pool: mysql::Pool,
}
impl YubanDatabase {
    pub fn new() -> Option<Self> {
        let opts = OptsBuilder::new()
            .ip_or_hostname(Some("127.0.0.1"))
            .tcp_port(3306)
            .user(Some("yubanmanager"))
            .db_name(Some("yuban"))
            .pass(Some("PajqMDXIloNcwuxG27udp3gy4EBi"));
        let pool = match mysql::Pool::new(opts) {
            Ok(pool) => pool,
            Err(err) => {
                dbg!(err);
                return None;
            }
        };
        Some(Self { pool })
    }

    fn get_conn(&self) -> Result<mysql::PooledConn, ()> {
        self.pool.try_get_conn(500).map_err(|_| ())
    }

    pub fn add_post(
        &self,
        username: &str,
        post: &str,
        thread_id: u64,
        langcode: &str,
    ) -> Result<u64, ()> {
        let mut conn = self.get_conn()?;

        let mut transaction = conn
            .start_transaction(mysql::TxOpts::default())
            .map_err(|err| {
                dbg!(err);
            })?;
        let user_id = transactional_yuban::get_user_id(username, &mut transaction)?;
        let post_id = transactional_yuban::post(user_id, post, &mut transaction)?;
        transactional_yuban::link_orig(thread_id, post_id, langcode, &mut transaction)
            .map(|_| post_id)?;

        transaction.commit().map_err(|err| {
            dbg!(err);
        })?;
        Ok(post_id)
    }

    pub fn add_new_thread(
        &self,
        username: &str,
        post: &str,
        langcode: &str,
    ) -> Result<(u64, u64), ()> {
        const STATEMENT_STRING: &str = concat!(
            "INSERT INTO Threads (owner_id, opened_on) ",
            "VALUES (:owner_id, CURRENT_TIMESTAMP) "
        );
        let mut conn = self.get_conn()?;
        let mut transaction = conn
            .start_transaction(mysql::TxOpts::default())
            .map_err(|err| {
                dbg!(err);
            })?;
        let user_id = transactional_yuban::get_user_id(username, &mut transaction)?;

        let statement = transaction.prep(STATEMENT_STRING).map_err(|err| {
            dbg!(err);
        })?;
        let params = params! {"owner_id" => user_id};
        transaction.exec_drop(statement, params).map_err(|err| {
            dbg!(err);
        })?;
        let thread_id = transaction.last_insert_id().ok_or(())?;
        let post_id = transactional_yuban::post(user_id, post, &mut transaction)?;
        transactional_yuban::link_orig(thread_id, post_id, langcode, &mut transaction)
            .map(|_| post_id)?;

        transaction.commit().map_err(|err| {
            dbg!(err);
        })?;
        Ok((thread_id, post_id))
    }

    pub fn add_correction(&self, username: &str, post: &str, orig_id: u64) -> Result<u64, ()> {
        const STATEMENT_STRING_ORIG_LINK: &str = concat!(
            "INSERT INTO Corrections (orig_id, post_id) ",
            "VALUES (:orig_id, :post_id)"
        );
        let mut conn = self.get_conn()?;
        let mut transaction = conn
            .start_transaction(mysql::TxOpts::default())
            .map_err(|err| {
                dbg!(err);
            })?;
        let user_id = transactional_yuban::get_user_id(username, &mut transaction)?;
        let postid = transactional_yuban::post(user_id, post, &mut transaction)?;

        let statement = transaction
            .prep(STATEMENT_STRING_ORIG_LINK)
            .map_err(|err| {
                dbg!(err);
            })?;
        let params = params! {"orig_id" => orig_id, "post_id" => postid};
        transaction.exec_drop(statement, params).map_err(|err| {
            dbg!(err);
        })?;
        transaction.commit().map_err(|err| {
            dbg!(err);
        })?;
        Ok(postid)
    }

    pub fn get_post(&self, postid: usize) -> Result<Post, ()> {
        const STATEMENT_STRING: &str =
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/queries/get_post.sql"));
        let mut conn = self.get_conn()?;
        let statement = conn.prep(STATEMENT_STRING).map_err(|err| {
            dbg!(err);
        })?;
        conn.exec_first(statement, params! {"postid" => postid})
            .map_err(|err| {
                dbg!(err);
            })
            .and_then(|p| p.ok_or(()))
    }

    pub fn list_original_posts(&self) -> Result<Vec<ThreadSummary>, ()> {
        const STATEMENT_STRING: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/queries/list_original_posts.sql"
        ));
        println!("{}", STATEMENT_STRING);
        let mut conn = self.get_conn()?;
        conn.query(STATEMENT_STRING).map_err(|err| {
            dbg!(err);
        })
    }

    pub fn add_token(&self, userid: usize) -> Result<AccessToken, ()> {
        let mut conn = self.get_conn()?;
        const STATEMENT_STRING: &str = concat!(
            "INSERT INTO Tokens (userid, token, issuedate) ",
            "VALUES (:userid, :token, CURRENT_TIMESTAMP) ",
            "ON DUPLICATE KEY UPDATE token = :token, issuedate = CURRENT_TIMESTAMP"
        );

        let statement = conn.prep(STATEMENT_STRING).map_err(|err| {
            dbg!(err);
        })?;
        let token = AccessToken::default();
        conn.exec_drop(
            statement,
            params! {
                "userid" => userid,
                "token" => Vec::from(token)
            },
        )
        .map_err(|_| ())?;

        Ok(token)
    }

    pub fn check_token(&self, username: &str, token: &AccessToken) -> bool {
        let mut conn = match self.get_conn() {
            Ok(conn) => conn,
            Err(_) => return false,
        };

        let statement = conn
            .prep(concat!(
                "SELECT 1 FROM Users INNER JOIN Tokens ",
                "ON Users.id=Tokens.userid ",
                "WHERE Users.username = :username AND Tokens.token = :token"
            ))
            .map_err(|err| dbg!(err));
        let statement = match statement {
            Ok(x) => x,
            Err(_) => return true,
        };
        let _query_result: Option<usize> = match conn.exec_first(
            statement,
            params! {"username" => username, "token" => token.as_ref() },
        ) {
            Ok(Some(x)) => x,
            Ok(None) => return false,
            Err(err) => {
                dbg!(err);
                return false;
            }
        };
        true
    }

    pub fn new_login(&self, name: &str, pass: &str) -> Result<(), ()> {
        let mut conn = self.get_conn()?;

        let lower_name = name.to_lowercase();

        let statement = conn
            .prep(concat!(
                "INSERT INTO Users (username, passwordHash, salt) ",
                "VALUES (:username, :pwh, :salt)"
            ))
            .map_err(|e| {
                dbg!(e);
            })?;

        let salt = generate_salt();
        let pwhash = salted_pw(pass, &salt);
        let params = params! {
            "username" => &lower_name,
            "pwh" => pwhash,
            "salt" => salt
        };
        conn.exec_drop(statement, params).map_err(|_| ())
    }

    pub fn check_login(&self, name: &str, pass: &str) -> Result<usize, ()> {
        let mut conn = self.get_conn()?;
        let statement = conn
            .prep("SELECT id, passwordHash, salt FROM Users WHERE username = :username")
            .map_err(|err| {
                dbg!(err);
            })?;

        let query_result: (usize, Vec<u8>, Vec<u8>) = conn
            .exec_first(statement, params! {"username" => name})
            .map_err(|err| {
                dbg!(err);
            })?
            .ok_or(())?;

        let (id, pwhash, salt) = query_result;
        let hash_result = salted_pw(pass, &salt);
        let hash_equal = compare_slice(&hash_result, &pwhash);
        hash_equal.then_some(id).ok_or(())
    }
}
