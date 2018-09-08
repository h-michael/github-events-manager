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

mod cmd;
mod db_utils;
mod init;
mod model;
mod query;
mod schema;

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
    Command::Test {} => cmd::test::token_test(),
    Command::Init {} => init::init(),
    Command::Add {
        repo_flag,
        repo_name,
    } => match repo_flag {
        true => cmd::add::add_repository(repo_name),
        _ => panic!("set repository name"),
    },
    Command::Import {} => cmd::import::import(),
    Command::List {} => cmd::list::show_repository_list(),
});
