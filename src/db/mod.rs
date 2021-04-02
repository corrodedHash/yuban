use crate::auth::AccessToken;
use mysql::{
    params,
    prelude::{ConvIr, FromValue, Queryable},
    OptsBuilder,
};
use rand::RngCore;

pub mod transactional;

#[derive(serde::Serialize, Debug, Clone)]
pub struct SkyDate(#[serde(serialize_with = "serialize_date")] pub mysql::Value);

impl ConvIr<SkyDate> for mysql::Value {
    fn new(v: mysql::Value) -> Result<Self, mysql::FromValueError> {
        Ok(v)
    }

    fn commit(self) -> SkyDate {
        SkyDate(self)
    }

    fn rollback(self) -> mysql::Value {
        self
    }
}

impl FromValue for SkyDate {
    type Intermediate = mysql::Value;
}

pub fn serialize_date<S: serde::Serializer>(x: &mysql::Value, s: S) -> Result<S::Ok, S::Error> {
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

fn generate_salt() -> Vec<u8> {
    let mut salt_vec = [0_u8; 16];
    rand::thread_rng().fill_bytes(&mut salt_vec);
    salt_vec.to_vec()
}

#[macro_export]
macro_rules! load_query {
    ($filename: tt) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/queries/", $filename))
    };
}

pub struct YubanDatabase {
    pool: mysql::Pool,
}
impl YubanDatabase {
    pub fn new(db_opts: OptsBuilder) -> Option<Self> {
        let pool = match mysql::Pool::new(db_opts) {
            Ok(pool) => pool,
            Err(err) => {
                dbg!(err);
                return None;
            }
        };
        Some(Self { pool })
    }

    pub fn get_conn(&self) -> Result<mysql::PooledConn, ()> {
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
        let user_id = transactional::get_user_id(username, &mut transaction)?;
        let post_id = transactional::post(user_id, post, &mut transaction)?;
        transactional::link_orig(thread_id, post_id, langcode, &mut transaction)
            .map(|_| post_id)?;

        transaction.commit().map_err(|err| {
            dbg!(err);
        })?;
        Ok(post_id)
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
        let user_id = transactional::get_user_id(username, &mut transaction)?;
        let postid = transactional::post(user_id, post, &mut transaction)?;

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

    pub fn list_users(&self) -> Result<Vec<String>, ()> {
        const STATEMENT_STRING: &str = "SELECT username FROM Users";
        let mut conn = self.get_conn()?;
        let statement = conn.prep(STATEMENT_STRING).map_err(|err| {
            dbg!(err);
        })?;
        conn.exec(statement, ()).map_err(|err| {
            dbg!(err);
        })
    }

    pub fn add_token(&self, userid: u64) -> Result<AccessToken, ()> {
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

    pub fn remove_token(&self, userid: u64, token: AccessToken) -> Result<(), ()> {
        let mut conn = self.get_conn()?;
        const STATEMENT_STRING: &str =
            concat!("DELETE FROM Tokens WHERE userid = :userid AND token = :token",);

        let statement = conn.prep(STATEMENT_STRING).map_err(|err| {
            dbg!(err);
        })?;
        conn.exec_drop(
            statement,
            params! {
                "userid" => userid,
                "token" => token.as_ref()
            },
        )
        .map_err(|_| ())?;

        Ok(())
    }

    pub fn check_token(&self, username: &str, token: &AccessToken) -> Result<u64, ()> {
        let mut conn = self.get_conn().map_err(|err| {
            dbg!(err);
        })?;

        let statement = conn
            .prep(concat!(
                "SELECT Users.id FROM Users INNER JOIN Tokens ",
                "ON Users.id=Tokens.userid ",
                "WHERE Users.username = :username AND Tokens.token = :token"
            ))
            .map_err(|err| dbg!(err));
        let statement = statement.map_err(|err| {
            dbg!(err);
        })?;
        let userid: u64 = conn
            .exec_first(
                statement,
                params! {"username" => username, "token" => token.as_ref() },
            )
            .map_err(|err| {
                dbg!(err);
            })?
            .ok_or(())?;
        Ok(userid)
    }

    pub fn new_login(&self, name: &str, pass: &str) -> Result<(), ()> {
        let mut conn = self.get_conn()?;

        let lower_name = name.to_lowercase();

        let statement = conn
            .prep(concat!(
                "INSERT INTO Users (username, passwordHash) ",
                "VALUES (:username, :pwh)"
            ))
            .map_err(|e| {
                dbg!(e);
            })?;

        let salt = generate_salt();
        let pwhash = argon2::hash_encoded(pass.as_bytes(), &salt, &argon2::Config::default())
            .map_err(|_| ())?;
        let params = params! {
            "username" => &lower_name,
            "pwh" => pwhash,
        };
        conn.exec_drop(statement, params).map_err(|_| ())
    }

    pub fn remove_login(&self, name: &str) -> Result<(), ()> {
        let mut conn = self.get_conn()?;

        let lower_name = name.to_lowercase();

        let statement = conn
            .prep(concat!("DELETE FROM Users ", "WHERE username = :username"))
            .map_err(|e| {
                dbg!(e);
            })?;

        let params = params! {
            "username" => &lower_name,
        };
        conn.exec_drop(statement, params).map_err(|_| ())
    }

    pub fn check_login(&self, name: &str, pass: &str) -> Result<u64, ()> {
        let mut conn = self.get_conn()?;
        let statement = conn
            .prep("SELECT id, passwordHash FROM Users WHERE username = :username")
            .map_err(|err| {
                dbg!(err);
            })?;

        let query_result: (u64, String) = conn
            .exec_first(statement, params! {"username" => name})
            .map_err(|err| {
                dbg!(err);
            })?
            .ok_or(())?;

        let (id, pwhash) = query_result;
        argon2::verify_encoded(&pwhash, pass.as_bytes())
            .map_err(|_| {
                dbg!("argon2 errored");
            })?
            .then_some(id)
            .ok_or(())
    }
}
