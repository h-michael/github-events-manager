use super::utils::*;
use db_utils::*;
use diesel;
use diesel::connection::Connection;
use diesel::query_dsl::*;
use diesel::result::Error;
use diesel::RunQueryDsl;
use graphql_client::*;
use model::*;
use query::issues;
use query::pull_requests;
use schema::issues as issue_model;
use schema::pull_requests as pr_model;
use schema::repositories::dsl::*;

pub fn fetch_all() {
    use diesel::ExpressionMethods;

    let connection = establish_connection();
    let results = repositories.load::<Repository>(&connection).unwrap();
    for repository in results {
        let (issues, issue_end_cursor) = fetch_issues(
            repository.id,
            repository.owner.clone(),
            repository.name.clone(),
            repository.last_issue_cursor.clone(),
            vec![],
        );
        println!("{:?}: issues {:?}", repository.name, issues.len());
        let (prs, pr_end_cursor) = fetch_prs(
            repository.id,
            repository.owner.clone(),
            repository.name.clone(),
            repository.last_pr_cursor.clone(),
            vec![],
        );

        println!("{:?}: pull requests {:?}", repository.name, prs.len());
        connection
            .transaction::<_, Error, _>(|| {
                diesel::update(repositories.filter(id.eq(repository.id)))
                    .set((
                        last_issue_cursor.eq(issue_end_cursor),
                        last_pr_cursor.eq(pr_end_cursor),
                    ))
                    .execute(&connection)?;
                diesel::insert_into(issue_model::table)
                    .values(issues)
                    .execute(&connection)?;
                diesel::insert_into(pr_model::table)
                    .values(prs)
                    .execute(&connection)?;
                Ok(())
            })
            .unwrap_or_else(|err| panic!("fetch failed: {:?}", err));
    }
}

fn fetch_issues(
    repository_id: i32,
    owner_value: String,
    name_value: String,
    after: Option<String>,
    mut issues: Vec<NewIssue>,
) -> (Vec<NewIssue>, Option<String>) {
    let query = issues::Issues::build_query(issues::Variables {
        owner: owner_value.clone(),
        name: name_value.clone(),
        first: Some(100),
        after,
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

    issues.extend(
        issue_edges
            .into_iter()
            .map(|issue| match issue {
                Some(issue) => {
                    let issue = issue.node.unwrap();
                    NewIssue {
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
                        last_event_cursor: None,
                    }
                }
                None => panic!(),
            })
            .collect::<Vec<_>>(),
    );

    if info.has_next_page {
        match info.end_cursor {
            Some(ref end_cursor) => fetch_issues(
                repository_id,
                owner_value,
                name_value,
                Some(end_cursor.clone()),
                issues,
            ),
            None => (issues, None),
        }
    } else {
        (issues, info.end_cursor.clone())
    }
}

fn fetch_prs(
    repository_id: i32,
    owner_value: String,
    name_value: String,
    after: Option<String>,
    mut pull_requests: Vec<NewPullRequest>,
) -> (Vec<NewPullRequest>, Option<String>) {
    let query = pull_requests::PullRequests::build_query(pull_requests::Variables {
        owner: owner_value.clone(),
        name: name_value.clone(),
        first: Some(100),
        after,
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

    pull_requests.extend(
        pull_request_edges
            .into_iter()
            .map(|pull_request| match pull_request {
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
                        last_event_cursor: None,
                    }
                }
                None => panic!(),
            })
            .collect::<Vec<_>>(),
    );

    if info.has_next_page {
        match info.end_cursor {
            Some(ref end_cursor) => fetch_prs(
                repository_id,
                owner_value,
                name_value,
                Some(end_cursor.clone()),
                pull_requests,
            ),
            None => (pull_requests, None),
        }
    } else {
        (pull_requests, info.end_cursor.clone())
    }
}
