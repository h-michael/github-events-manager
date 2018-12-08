use crate::db_utils::establish_connection;
use diesel::prelude::*;
use crate::model::Repository;
use crate::schema::repositories::dsl::repositories;

pub fn show_repository_list() {
    let connection = establish_connection();

    let results = repositories
        .load::<Repository>(&connection)
        .expect("Error loading posts");

    for repository in results {
        println!("{}/{}", repository.owner, repository.name);
    }
}
