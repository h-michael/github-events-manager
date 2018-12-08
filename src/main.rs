#![allow(proc_macro_derive_resolution_fallback)]
use quicli::prelude::*;
use structopt::StructOpt;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use graphql_client;

use reqwest;

use serde;

#[macro_use]
extern crate serde_derive;

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
    #[structopt(name = "sync")]
    Sync {
        #[structopt(short = "w", long = "watching")]
        watching: bool,
        #[structopt(short = "s", long = "stars")]
        stars: bool,
    },
    #[structopt(name = "list")]
    List {},
    #[structopt(name = "fetch")]
    Fetch {},
}

fn main() -> CliResult {
    let args = Ghe::from_args();
    match &args.cmd {
        Command::Test {} => cmd::test::token_test(),
        Command::Init {} => init::init(),
        Command::Add {
            repo_flag,
            repo_name,
        } => match repo_flag {
            true => cmd::add::add_repository(repo_name),
            _ => panic!("set repository name"),
        },
        Command::Sync { watching, stars } => cmd::sync::sync(),
        Command::List {} => cmd::list::show_repository_list(),
        Command::Fetch {} => cmd::fetch::fetch_all(),
    }
    Ok(())
}
