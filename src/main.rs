#![feature(extern_prelude)]
#[macro_use]
extern crate quicli;
use quicli::prelude::*;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate graphql_client;

extern crate reqwest;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate dotenv;

mod action;
mod db_utils;
mod init;
mod repository_query;
mod resources;
mod schema;
mod test_query;
mod watch_query;

#[derive(StructOpt)]
#[structopt(name = "ghe", about = "Github events manager")]
struct Ghe {
    #[structopt(subcommand)]
    cmd: Command,
}
#[derive(StructOpt)]
enum Command {
    #[structopt(name = "test")]
    Test {},
    #[structopt(name = "init")]
    Init {},
    #[structopt(name = "add")]
    Add {
        #[structopt(short = "r", long = "repository")]
        repo_flag: bool,
        repo_name: String,
    },
    #[structopt(name = "import")]
    Import {},
    #[structopt(name = "list")]
    List {},
}

main!(|args: Ghe| match &args.cmd {
    Command::Test {} => action::token_test(),
    Command::Init {} => init::init(),
    Command::Add {
        repo_flag,
        repo_name,
    } => match repo_flag {
        true => action::add_repository(repo_name),
        _ => panic!("set repository name"),
    },
    Command::Import {} => action::import(),
    Command::List {} => action::show_repository_list(),
});
