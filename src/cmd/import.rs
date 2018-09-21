use super::utils::*;
use db_utils::*;
use diesel;
use diesel::connection::Connection;
use diesel::expression_methods::*;
use diesel::query_dsl::QueryDsl;
use diesel::result::Error;
use diesel::RunQueryDsl;
use graphql_client::*;
use model::*;
use query::watch;
use schema::issue_event_conditions;
use schema::pull_request_event_conditions;
use schema::repositories;

pub fn import() {
    let repository_results = request_watching_repositories(None, None);
    let connection = establish_connection();

    connection
        .transaction::<_, Error, _>(|| {
            diesel::insert_into(repositories::table)
                .values(repository_results.clone())
                .execute(&connection)?;

            let ids = repositories::table
                .select(repositories::id)
                .filter(
                    repositories::repository_id.eq_any(
                        repository_results
                            .clone()
                            .iter()
                            .map(|repo| &repo.repository_id),
                    ),
                ).load::<i32>(&connection)?;

            let mut new_issue_event_conditions = Vec::new();
            let mut new_pull_request_event_conditions = Vec::new();

            for id in ids.iter() {
                new_issue_event_conditions.append(&mut vec![NewIssueEventCondition {
                    repository_id: id.clone() as i32,
                    start_condition: 7,
                    stop_condition: 1,
                }]);

                new_pull_request_event_conditions.append(&mut vec![NewPullRequestEventCondition {
                    repository_id: id.clone() as i32,
                    start_condition: 31,
                    stop_condition: 3,
                }]);
            }

            diesel::insert_into(issue_event_conditions::table)
                .values(new_issue_event_conditions)
                .execute(&connection)?;

            diesel::insert_into(pull_request_event_conditions::table)
                .values(new_pull_request_event_conditions)
                .execute(&connection)?;
            Ok(())
        }).expect("Import failed");
}

fn request_watching_repositories<'a>(
    after: Option<String>,
    repositories: Option<&'a mut Vec<NewRepository>>,
) -> Vec<NewRepository> {
    let q = watch::WatchQuery::build_query(watch::Variables {
        first: 100,
        after: after,
    });
    let res = request(&q);
    let watching = match res {
        Ok(mut res) => {
            let body: Response<watch::ResponseData> = match res.json() {
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
