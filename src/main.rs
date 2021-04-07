#![feature(proc_macro_hygiene, decl_macro)]
#![feature(bool_to_option)]

mod auth;
mod db;
mod routes;
use std::path::PathBuf;

use rocket::Rocket;

use clap::Clap;

const STATIC_FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/webpage/dist");
pub struct StaticFilePath(PathBuf);
#[derive(Clap)]
#[clap(about = r#"MySQL password passed by environment variable 'YUBAN_MYSQL_PASSWORD'"#)]
struct Opts {
    #[clap(short, long)]
    complete: bool,
    #[clap(short, long, default_value=STATIC_FILE_PATH)]
    static_file_path: String,

    #[clap(long, default_value = "127.0.0.1")]
    mysql_host: String,
    #[clap(long, default_value = "3306")]
    mysql_port: u16,
    #[clap(short = 'u', long, default_value = "yubanmanager")]
    mysql_user: String,

    #[clap(long)]
    add_user: Option<String>,
    #[clap(short = 'a', long, default_value = "127.0.0.1")]
    bind_address: String,
    #[clap(short = 'p', long, default_value = "8000")]
    bind_port: u16,
}
fn standard_server(rocket: Rocket) -> Rocket {
    rocket
        .mount("/r", routes::get_fix_routes())
        .mount("/api", routes::get_api_routes())
        .mount("/api/admin", routes::get_admin_routes())
}

fn complete_server(rocket: Rocket, static_file_path: PathBuf) -> Rocket {
    standard_server(rocket)
        .mount(
            "/",
            rocket_contrib::serve::StaticFiles::from(static_file_path.clone()),
        )
        .manage(StaticFilePath(static_file_path))
}

fn main() {
    let cmdline_opts = Opts::parse();

    let mut db_tries = 0;
    let db = loop {
        if db_tries > 5 {
            eprintln!("Could not connect to mysql server");
            std::process::exit(1);
        }
        db_tries += 1;
        let mysql_opts = mysql::OptsBuilder::new()
            .ip_or_hostname(Some(cmdline_opts.mysql_host.clone()))
            .tcp_port(cmdline_opts.mysql_port)
            .user(Some(cmdline_opts.mysql_user.clone()))
            .db_name(Some("yuban"))
            .pass(Some(std::env::var("YUBAN_MYSQL_PASSWORD").expect(
                "Specify environment variable 'YUBAN_MYSQL_PASSWORD'",
            )));

        if let Some(db) = db::YubanDatabase::new(mysql_opts) {
            break db;
        }
        std::thread::sleep(std::time::Duration::new(3, 0));
    };

    if let Some(username) = cmdline_opts.add_user {
        if let Ok(password) = std::env::var("YUBAN_ADD_USER_PW") {
            db.new_login(&username, &password)
                .expect("Could not add user");
            return;
        } else {
            eprintln!(
                "Please define YUBAN_ADD_USER_PW environment variable as password for {}",
                username
            );
            std::process::exit(1);
        }
    }

    use rocket::config::{Config, Environment};
    let config = Config::build(Environment::Staging)
        .address(cmdline_opts.bind_address)
        .port(cmdline_opts.bind_port)
        .finalize()
        .unwrap();

    let r: Rocket = if cmdline_opts.complete {
        complete_server(
            rocket::custom(config),
            PathBuf::from(cmdline_opts.static_file_path),
        )
    } else {
        standard_server(rocket::custom(config))
    };

    r.manage(db).launch();
}
