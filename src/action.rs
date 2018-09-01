extern crate github_gql as gh;
extern crate serde_json;

use self::gh::client::Github;
use self::gh::mutation::Mutation;
use self::gh::query::Query;
use self::serde_json::Value;
use diesel;
use diesel::RunQueryDsl;
use dotenv::dotenv;
use std::env;

use db_utils::*;
use resources::*;

fn get_client() -> gh::client::Github {
    dotenv().ok();
    let token =
        env::var("GITHUB_ENVENTS_MANAGER_TOKEN").expect("GITHUB_ENVENTS_MANAGER_TOKEN must be set");
    Github::new(token).unwrap()
}

pub fn token_test() {
    let mut client = get_client();
    let (_, _, json) = client
        .query::<Value>(&Query::new_raw("query { viewer { login } }"))
        .unwrap();

    if let Some(json) = json {
        println!("{}", json);
    }
}

pub fn add_repository(repo_name: &String) {
    use super::schema::repositories;

    let splitted: Vec<&str> = repo_name.split_terminator("/").collect();

    if splitted.len() != 2 {
        panic!("Argument must be formated with \"owner/name\" ");
    }
    let owner = splitted[0];
    let name = splitted[1];

    if !owner.is_empty() && !name.is_empty() {
        let connection = establish_connection();
        let new_repository = NewRepository {
            owner: owner,
            name: name,
        };

        diesel::insert_into(repositories::table)
            .values(&new_repository)
            .execute(&connection)
            .expect("Error saving new post");
    }
}
