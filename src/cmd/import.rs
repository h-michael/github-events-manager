use super::utils::*;
use db_utils::*;
use diesel::RunQueryDsl;
use graphql_client::*;
use model::*;
use query::watch;
use schema::repositories;

pub fn import() {
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
    repositories: Option<&'a mut Vec<NewRepository>>,
) -> Vec<NewRepository> {
    let q = watch::WatchQuery::build_query(watch::watch_query::Variables {
        first: 100,
        after: after,
    });
    let res = request(&q);
    let watching = match res {
        Ok(mut res) => {
            let body: GraphQLResponse<watch::watch_query::ResponseData> = match res.json() {
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
