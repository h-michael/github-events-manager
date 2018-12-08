use crate::schema::issue_event_conditions;
use crate::schema::issues;
use crate::schema::pull_request_event_conditions;
use crate::schema::pull_requests;
use crate::schema::repositories;

#[derive(Debug, Queryable, PartialEq)]
pub struct Repository {
    pub id: i32,
    pub node_id: String,
    pub owner: String,
    pub name: String,
    pub url: String,
    pub last_pr_cursor: Option<String>,
    pub last_issue_cursor: Option<String>,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "repositories"]
pub struct NewRepository {
    pub node_id: String,
    pub owner: String,
    pub name: String,
    pub url: String,
    pub last_pr_cursor: Option<String>,
    pub last_issue_cursor: Option<String>,
}

#[derive(Debug, Identifiable, Queryable, Associations)]
#[belongs_to(Repository)]
pub struct PullRequestEventCondition {
    pub id: i32,
    pub repository_id: String,
    pub start_condition: i32,
    pub stop_condition: i32,
    pub listen_status: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "pull_request_event_conditions"]
pub struct NewPullRequestEventCondition {
    pub repository_id: i32,
    pub start_condition: i32,
    pub stop_condition: i32,
    pub listen_status: i32,
}

#[derive(Debug, Identifiable, Queryable, Associations)]
#[belongs_to(Repository)]
pub struct IssueEventCondition {
    pub id: i32,
    pub repository_id: String,
    pub start_condition: i32,
    pub stop_condition: i32,
    pub listen_status: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "issue_event_conditions"]
pub struct NewIssueEventCondition {
    pub repository_id: i32,
    pub start_condition: i32,
    pub stop_condition: i32,
    pub listen_status: i32,
}

#[derive(Debug, Queryable, Associations)]
#[belongs_to(Repository)]
pub struct Issue {
    pub id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub edited_at: Option<String>,
    pub closed_at: Option<String>,
    pub node_id: String,
    pub number: i32,
    pub repository_id: i32,
    pub state: String,
    pub title: Option<String>,
    pub body: Option<String>,
    pub closed: i32,
    pub last_event_cursor: Option<String>,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "issues"]
pub struct NewIssue {
    pub created_at: String,
    pub updated_at: String,
    pub edited_at: Option<String>,
    pub closed_at: Option<String>,
    pub node_id: String,
    pub number: i32,
    pub repository_id: i32,
    pub state: String,
    pub title: Option<String>,
    pub body_text: Option<String>,
    pub closed: i32,
    pub last_event_cursor: Option<String>,
}

#[derive(Debug, Queryable, Associations)]
#[belongs_to(Repository)]
pub struct PullRequest {
    pub id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub edited_at: Option<String>,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    pub node_id: String,
    pub number: i32,
    pub repository_id: i32,
    pub state: String,
    pub title: Option<String>,
    pub body: Option<String>,
    pub closed: i32,
    pub merged: i32,
    pub last_event_cursor: Option<String>,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "pull_requests"]
pub struct NewPullRequest {
    pub created_at: String,
    pub updated_at: String,
    pub edited_at: Option<String>,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    pub node_id: String,
    pub number: i32,
    pub repository_id: i32,
    pub state: String,
    pub title: Option<String>,
    pub body_text: Option<String>,
    pub closed: i32,
    pub merged: i32,
    pub last_event_cursor: Option<String>,
}
