use diesel;
use diesel::RunQueryDsl;
use dotenv::dotenv;
use graphql_client::*;
use reqwest;
use std::env;

use db_utils::*;
use repository_query;
use resources::*;
use test_query;

pub fn token_test() {
    dotenv().ok();
    let token =
        env::var("GITHUB_ENVENTS_MANAGER_TOKEN").expect("GITHUB_ENVENTS_MANAGER_TOKEN must be set");
    let q = test_query::TestQuery::build_query(test_query::test_query::Variables {});
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.github.com/graphql")
        .header(reqwest::header::Authorization(format!("bearer {}", token)))
        .json(&q)
        .send();

    match res {
        Ok(mut res) => {
            let body: GraphQLResponse<test_query::test_query::ResponseData> = res.json().expect("");
            println!("{:?}", body);
        }
        Err(e) => println!("{:?}", e),
    };
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
        dotenv().ok();
        let token = env::var("GITHUB_ENVENTS_MANAGER_TOKEN")
            .expect("GITHUB_ENVENTS_MANAGER_TOKEN must be set");
        let q = repository_query::RepositoryQuery::build_query(
            repository_query::repository_query::Variables {
                owner: owner.to_string(),
                name: name.to_string(),
            },
        );
        let client = reqwest::Client::new();
        let res = client
            .post("https://api.github.com/graphql")
            .header(reqwest::header::Authorization(format!("bearer {}", token)))
            .json(&q)
            .send();

        let repository = match res {
            Ok(mut res) => {
                let body: GraphQLResponse<
                    repository_query::repository_query::ResponseData,
                > = res.json().unwrap();
                body.data.unwrap().repository.unwrap()
            }
            Err(e) => panic!("{:?}", e),
        };

        let repository_id = repository.id.clone();
        let url = repository.url.clone();
        let connection = establish_connection();
        let new_repository = NewRepository {
            owner,
            name,
            repository_id: repository_id.as_str(),
            url: url.as_str(),
        };

        diesel::insert_into(repositories::table)
            .values(&new_repository)
            .execute(&connection)
            .expect("Error saving new repository");
    }
}

pub fn show_repository_list() {
    use super::diesel::prelude::*;
    use super::schema::repositories::dsl::*;
    let connection = establish_connection();

    let results = repositories
        .filter(owner.eq("h-michael"))
        .load::<Repository>(&connection)
        .expect("Error loading posts");

    for repository in results {
        println!("{}/{}", repository.owner, repository.name);
    }
}
