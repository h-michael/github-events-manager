use super::utils::*;
use db_utils::*;
use diesel;
use diesel::connection::Connection;
use diesel::result::Error;
use diesel::RunQueryDsl;
use graphql_client::*;
use model::*;
use query::issues;
use query::pull_requests;
use schema::issues as issue_model;
use schema::pull_requests as pr_model;
use schema::repositories::dsl::repositories;
// use schema::issue_event_conditions;
// use schema::pull_request_event_conditions;

pub fn fetch_all() {
    let connection = establish_connection();
    let results = repositories.load::<Repository>(&connection).unwrap();
    for repository in results {
        let issues = fetch_issues(repository.id, repository.owner.clone(), repository.name.clone());
        let prs = fetch_prs(repository.id, repository.owner.clone(), repository.name.clone());

        connection
            .transaction::<_, Error, _>(|| {
                diesel::insert_into(issue_model::table).values(issues.clone())
                    .execute(&connection)?;
                diesel::insert_into(pr_model::table).values(prs.clone())
                    .execute(&connection)?;
                Ok(())
            }).unwrap_or_else(|err| panic!("fetch failed: {:?}", err));

    }

}

fn fetch_issues(repository_id: i32, owner: String, name: String) -> Vec<NewIssue> {
    let query = issues::Issues::build_query(issues::Variables {
        owner,
        name,
        first: Some(100),
        states: Some(vec![issues::IssueState::OPEN]),
    });
    let res = request(&query);
    let repository = match res {
        Ok(mut res) => {
            let body: Response<issues::ResponseData> = match res.json() {
                Ok(body) => body,
                Err(e) => panic!("{}", e),
            };
            body.data.unwrap().repository.unwrap()
        }
        Err(e) => panic!("{:?}", e),
    };

    let info = &repository.issues.page_info;
    let issue_edges = repository.issues.edges.unwrap();

    issue_edges.into_iter().map(|issue| match issue {
        Some(issue) => {
            let issue = issue.node.unwrap();
            return NewIssue {
                closed: issue.closed(),
                created_at: issue.created_at,
                updated_at: issue.updated_at,
                edited_at: issue.last_edited_at,
                closed_at: issue.closed_at,
                node_id: issue.id,
                number: issue.number as i32,
                repository_id,
                state: match issue.state {
                    issues::IssueState::CLOSED => "CLOSED".to_string(),
                    issues::IssueState::OPEN => "OPEN".to_string(),
                    issues::IssueState::Other(state) => state,
                },
                title: Some(issue.title),
                body_text: Some(issue.body_text),
                last_issue_cursor: info.end_cursor.to_owned(),
            }
        },
        None => panic!(),
    }).collect::<Vec<_>>()
}

fn fetch_prs(repository_id: i32, owner: String, name: String) -> Vec<NewPullRequest>{
    let query = pull_requests::PullRequests::build_query(pull_requests::Variables {
        owner,
        name,
        first: Some(100),
        states: Some(vec![pull_requests::PullRequestState::OPEN]),
    });
    let res = request(&query);

    let repository = match res {
        Ok(mut res) => {
            let body: Response<pull_requests::ResponseData> = match res.json() {
                Ok(body) => body,
                Err(e) => panic!("{}", e),
            };
            body.data.unwrap().repository.unwrap()
        }
        Err(e) => panic!("{:?}", e),
    };

    let info = &repository.pull_requests.page_info;
    let pull_request_edges = repository.pull_requests.edges.unwrap();

    pull_request_edges.into_iter().map(|pull_request| match pull_request {
        Some(pull_request) => {
            let pull_request = pull_request.node.unwrap();
            NewPullRequest {
                closed: pull_request.closed(),
                merged: pull_request.merged(),
                created_at: pull_request.created_at,
                updated_at: pull_request.updated_at,
                edited_at: pull_request.last_edited_at,
                closed_at: pull_request.closed_at,
                merged_at: pull_request.merged_at,
                node_id: pull_request.id,
                number: pull_request.number as i32,
                repository_id,
                state: match pull_request.state {
                    pull_requests::PullRequestState::MERGED => "MERGED".to_string(),
                    pull_requests::PullRequestState::CLOSED => "CLOSED".to_string(),
                    pull_requests::PullRequestState::OPEN => "OPEN".to_string(),
                    pull_requests::PullRequestState::Other(state) => state,
                },
                title: Some(pull_request.title),
                body_text: Some(pull_request.body_text),
                last_pull_request_cursor: info.end_cursor.to_owned(),
            }
        },
        None => panic!(),
    }).collect::<Vec<_>>()
}
