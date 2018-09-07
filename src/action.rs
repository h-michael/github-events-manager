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
use watch_query;

pub fn token_test() {
    let q = test_query::TestQuery::build_query(test_query::test_query::Variables {});
    let res = request(&q);

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
        let q = repository_query::RepositoryQuery::build_query(
            repository_query::repository_query::Variables {
                owner: owner.to_string(),
                name: name.to_string(),
            },
        );
        let res = request(&q);

        let repository = match res {
            Ok(mut res) => {
                let body: GraphQLResponse<
                    repository_query::repository_query::ResponseData,
                > = match res.json() {
                    Ok(body) => body,
                    Err(e) => panic!("{}", e),
                };
                body.data.unwrap().repository.unwrap()
            }
            Err(e) => panic!("{:?}", e),
        };

        let connection = establish_connection();
        let new_repository = NewRepository {
            owner: owner.to_string(),
            name: name.to_string(),
            repository_id: repository.id,
            url: repository.url,
        };

        diesel::insert_into(repositories::table)
            .values(&new_repository)
            .execute(&connection)
            .expect("Error saving new repository");
    }
}

pub fn import() {
    use super::schema::repositories;
    let repositories = request_watching_repositories(None, None);
    let connection = establish_connection();

    for repo in repositories.iter() {
        diesel::insert_into(repositories::table)
            .values(repo)
            .execute(&connection)
            .expect("Error saving new repository");
    }
}

fn request_watching_repositories<'a>(
    after: Option<String>,
    repositories: Option<&'a mut std::vec::Vec<NewRepository>>,
) -> std::vec::Vec<NewRepository> {
    let q = watch_query::WatchQuery::build_query(watch_query::watch_query::Variables {
        first: 100,
        after: after,
    });
    let res = request(&q);
    let watching = match res {
        Ok(mut res) => {
            let body: GraphQLResponse<watch_query::watch_query::ResponseData> = match res.json() {
                Ok(body) => body,
                Err(e) => panic!("{}", e),
            };
            body.data.unwrap().viewer.watching
        }
        Err(e) => panic!("{:?}", e),
    };

    let nodes = watching.nodes.unwrap();
    let mut new_repositories = nodes
        .iter()
        .map(|node| match node {
            Some(node) => {
                let splitted: Vec<&str> = node.name_with_owner.split_terminator("/").collect();
                let owner = splitted[0].to_string();
                let name = splitted[1].to_string();

                if !owner.is_empty() && !name.is_empty() {
                    return NewRepository {
                        owner,
                        name,
                        repository_id: node.id.clone(),
                        url: node.url.clone(),
                    };
                }
                panic!()
            }
            None => panic!(),
        }).collect::<Vec<_>>();

    if let Some(repositories) = repositories {
        new_repositories.append(repositories);
    }
    if watching.page_info.has_next_page {
        return request_watching_repositories(
            watching.page_info.end_cursor,
            Some(&mut new_repositories),
        );
    }
    new_repositories
}

pub fn show_repository_list() {
    use super::diesel::prelude::*;
    use super::schema::repositories::dsl::*;
    let connection = establish_connection();

    let results = repositories
        .load::<Repository>(&connection)
        .expect("Error loading posts");

    for repository in results {
        println!("{}/{}", repository.owner, repository.name);
    }
}

fn get_token() -> String {
    dotenv().ok();
    env::var("GITHUB_ENVENTS_MANAGER_TOKEN").expect("GITHUB_ENVENTS_MANAGER_TOKEN must be set")
}

fn request<T: super::serde::Serialize>(
    query: &GraphQLQueryBody<T>,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .post("https://api.github.com/graphql")
        .header(reqwest::header::Authorization(format!(
            "bearer {}",
            get_token()
        ))).json(query)
        .send()
}
