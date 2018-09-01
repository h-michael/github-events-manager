#[macro_use]
extern crate quicli;
use quicli::prelude::*;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

extern crate dotenv;

mod action;
mod db_utils;
mod init;
mod resources;
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
});
