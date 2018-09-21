use super::utils::*;
use db_utils::*;
use diesel;
use diesel::RunQueryDsl;
use graphql_client::*;
use model::*;
use query::repository;
use schema::repositories;

pub fn add_repository(repo_name: &String) {
    let splitted: Vec<&str> = repo_name.split_terminator("/").collect();

    if splitted.len() != 2 {
        panic!("Argument must be formated with \"owner/name\" ");
    }

    let owner = splitted[0];
    let name = splitted[1];

    if !owner.is_empty() && !name.is_empty() {
        let q = repository::Repository::build_query(repository::Variables {
            owner: owner.to_string(),
            name: name.to_string(),
        });
        let res = request(&q);

        let repository = match res {
            Ok(mut res) => {
                let body: Response<repository::ResponseData> = match res.json() {
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
