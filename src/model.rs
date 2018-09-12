use super::schema::issue_event_conditions;
use super::schema::pull_request_event_conditions;
use super::schema::repositories;

#[derive(Debug, Queryable)]
pub struct Repository {
    pub id: i32,
    pub repository_id: String,
    pub owner: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "repositories"]
pub struct NewRepository {
    pub owner: String,
    pub repository_id: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Queryable)]
pub struct PullRequestEventCondition {
    pub id: i32,
    pub repository_id: String,
    pub start_condition: i32,
    pub stop_condition: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "pull_request_event_conditions"]
pub struct NewPullRequestEventCondition {
    pub repository_id: i32,
    pub start_condition: i32,
    pub stop_condition: i32,
}

#[derive(Debug, Queryable)]
pub struct IssueEventCondition {
    pub id: i32,
    pub repository_id: String,
    pub start_condition: i32,
    pub stop_condition: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "issue_event_conditions"]
pub struct NewIssueEventCondition {
    pub repository_id: i32,
    pub start_condition: i32,
    pub stop_condition: i32,
}
