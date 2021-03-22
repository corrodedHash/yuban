use crate::auth::AccessToken;
use mysql::{
    params,
    prelude::{FromRow, Queryable},
    OptsBuilder,
};
use rand::RngCore;
use sha2::{digest::FixedOutput, Digest, Sha256};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Post {
    pub id: usize,
    pub posttime: String,
    pub text: String,
    pub user: String,
}

impl FromRow for Post {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (id, posttime, text, user) = mysql::prelude::FromRow::from_row_opt(row)?;
        Ok(Self {
            id,
            posttime,
            text,
            user,
        })
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

    pub fn get_post(&self, postid: usize) -> Result<Post, ()> {
        const STATEMENT_STRING: &str = concat!(
            "SELECT Posts.id, Posts.postdate, Posts.post, Users.username ",
            "FROM Posts INNER JOIN Users ",
            "ON Posts.userid = Users.id ",
            "WHERE Posts.id = :postid"
        );

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

    pub fn list_posts(&self) -> Result<Vec<Post>, ()> {
        const STATEMENT_STRING: &str = concat!(
            "SELECT Posts.id, Posts.postdate, SUBSTRING(Posts.post,0,10), Users.username ",
            "FROM Posts INNER JOIN Users ",
            "ON Posts.userid = Users.id ",
        );

        let mut conn = self.get_conn()?;
        conn.query(STATEMENT_STRING).map_err(|err| {
            dbg!(err);
        })
    }

    pub fn add_token(&self, userid: usize) -> Result<AccessToken, ()> {
        let mut conn = self.get_conn()?;
        const STATEMENT_STRING: &str = concat!(
            "INSERT INTO Tokens (userid, token, issuedate) ",
            "VALUES (:userid,:token, CURRENT_TIMESTAMP) ",
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
