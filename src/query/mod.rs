pub mod issue_timeline_items;
pub mod issues;
pub mod login_user;
pub mod pull_request_review;
pub mod pull_request_timeline_items;
pub mod pull_requests;
pub mod rate_limit;
pub mod repository;
pub mod typedef;
pub mod watching_repositories;

impl pull_requests::RustPullRequestsRepositoryPullRequestsEdgesNode {
    pub fn closed(&self) -> i32 {
        if self.closed { 1 } else { 0 }
    }

    pub fn merged(&self) -> i32 {
        if self.merged { 1 } else { 0 }
    }
}

impl issues::RustIssuesRepositoryIssuesEdgesNode {
    pub fn closed(&self) -> i32 {
        if self.closed { 1 } else { 0 }
    }
}
