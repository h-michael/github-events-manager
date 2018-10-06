#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use graphql_client;
use query::typedef::*;
use serde;

pub const QUERY : & 'static str = "fragment custom_commit on Commit {\n  id\n  oid\n  messageBody\n  messageHeadline\n  commitUrl\n  committedDate\n  pushedDate\n}\n\nfragment custom_actor on Actor {\n  __typename\n  ... on User {\n    id\n    name\n    login\n  }\n  ... on Organization {\n    id\n    name\n    login\n  }\n  ... on Bot {\n    id\n    login\n  }\n}\n\nfragment RateLimitInfo on Query {\n    rateLimit {\n        cost\n        limit\n        remaining\n        resetAt\n    }\n}\n\nquery LoginUser {\n  viewer {\n    login\n  }\n  ...RateLimitInfo\n}\n\nquery RateLimit {\n  ...RateLimitInfo\n}\n\nquery Repository($owner: String!, $name: String!) {\n  repository(owner: $owner, name: $name) {\n    id\n    url\n  }\n  ...RateLimitInfo\n}\n\nquery PullRequests($owner: String!, $name: String!, $first: Int = 100, $states: [PullRequestState!]) {\n  repository(owner: $owner, name: $name) {\n    id\n    url\n    pullRequests(first: $first, states: $states) {\n      edges {\n        node {\n          id\n          number\n          state\n          title\n          bodyText\n          createdAt\n          updatedAt\n          lastEditedAt\n          closed\n          closedAt\n          merged\n          mergedAt\n          repository {\n            id\n          }\n        }\n      }\n      pageInfo {\n        startCursor\n        endCursor\n        hasNextPage\n        hasPreviousPage\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery Issues($owner: String!, $name: String!, $first: Int = 100, $states: [IssueState!]){\n  repository(owner: $owner, name: $name) {\n    id\n    url\n    issues(first: $first, states: $states) {\n      edges {\n        node {\n          id\n          number\n          state\n          title\n          bodyText\n          createdAt\n          updatedAt\n          lastEditedAt\n          closed\n          closedAt\n          repository {\n            id\n          }\n        }\n      }\n      pageInfo {\n        startCursor\n        endCursor\n        hasNextPage\n        hasPreviousPage\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery WatchingRepositories($first: Int!, $after: String){\n  viewer {\n    watching(after: $after, first: $first) {\n      nodes {\n        id,\n        nameWithOwner,\n        url\n      }\n      pageInfo {\n        hasNextPage,\n        endCursor\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery PullRequestTimelineItems($id: ID!, $first: Int = 100) {\n  node(id: $id) {\n    __typename\n    ... on PullRequest {\n      id\n      author {\n        __typename\n        ...custom_actor\n      }\n      editor {\n        __typename\n        ...custom_actor\n      }\n      title\n      body\n      bodyText\n      timeline(first: $first) {\n        edges {\n          node {\n            __typename\n            ... on Commit {\n              ...custom_commit\n            }\n            ... on CommitCommentThread {\n              id\n              commit_of_comment: commit {\n                ...custom_commit\n              }\n            }\n            ... on PullRequestReview {\n              id\n              author {\n                __typename\n                ...custom_actor\n              }\n              commit_of_pr_review: commit {\n                ...custom_commit\n              }\n              state\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on PullRequestReviewThread {\n              id\n              comments(first: $first) {\n                edges {\n                  node {\n                    id\n                    bodyText\n                    createdAt\n                    updatedAt\n                    lastEditedAt\n                  }\n                }\n              }\n            }\n            ... on PullRequestReviewComment {\n              id\n              bodyText\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on IssueComment {\n              id\n              bodyText\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on ClosedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on ReopenedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on AssignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on UnassignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on LabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n            ... on ReviewRequestedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              requestedReviewer {\n                __typename\n                ...on User {\n                  id\n                  userName: name\n                  userLogin: login\n                }\n                ...on Team {\n                  id\n                  teamName: name\n                }\n              }\n              createdAt\n            }\n            ... on ReviewRequestRemovedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              requestedReviewer {\n                __typename\n                ...on User {\n                  id\n                  userName: name\n                  userLogin: login\n                }\n                ...on Team {\n                  id\n                  teamName: name\n                }\n              }\n              createdAt\n            }\n          }\n        }\n        pageInfo {\n          hasNextPage\n          endCursor\n        }\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery PullRequestReview($id: ID!, $first: Int = 100) {\n  node(id: $id) {\n    __typename\n    ... on PullRequestReview {\n      comments(first: $first) {\n        nodes {\n          id\n          bodyText\n          viewerDidAuthor\n        }\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery IssueTimelineItems($id: ID!, $first: Int = 100) {\n  node(id: $id) {\n    __typename\n    ... on Issue {\n      id\n      author {\n        __typename\n        ...custom_actor\n      }\n      editor {\n        __typename\n        ...custom_actor\n      }\n      title\n      body\n      bodyText\n      timeline(first: $first) {\n        edges {\n          node {\n            __typename\n            ... on Commit {\n              ...custom_commit\n            }\n            ... on IssueComment {\n              id\n              author {\n                __typename\n                ...custom_actor\n              }\n              bodyText\n              createdAt\n              lastEditedAt\n              publishedAt\n            }\n            ... on ClosedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              url\n              resourcePath\n              createdAt\n            }\n            ... on ReopenedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on AssignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on UnassignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on LabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n            ... on UnlabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n          }\n        }\n        pageInfo {\n          hasNextPage\n          endCursor\n        }\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\n# # FullAttributes\n# query PullRequestTimelineItems($id: ID!, $first: Int = 100) {\n#   node(id: $id) {\n#     __typename\n#     ... on PullRequest {\n#       id\n#       author {\n#         __typename\n#         ...custom_actor\n#       }\n#       editor {\n#         __typename\n#         ...custom_actor\n#       }\n#       title\n#       body\n#       bodyText\n#       timeline(first: $first) {\n#         edges {\n#           node {\n#             __typename\n#             ... on Commit {\n#               ...custom_commit\n#             }\n#             ... on CommitCommentThread {\n#               id\n#               commit_of_comment: commit {\n#                 ...custom_commit\n#               }\n#             }\n#             ... on PullRequestReview {\n#               id\n#               author {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               commit_of_pr_review: commit {\n#                 ...custom_commit\n#               }\n#               state\n#               createdAt\n#               updatedAt\n#               lastEditedAt\n#             }\n#             ... on PullRequestReviewThread {\n#               id\n#               comments(first: $first) {\n#                 edges {\n#                   node {\n#                     id\n#                     bodyText\n#                     createdAt\n#                     updatedAt\n#                     lastEditedAt\n#                   }\n#                 }\n#               }\n#             }\n#             ... on PullRequestReviewComment {\n#               id\n#               bodyText\n#               createdAt\n#               updatedAt\n#               lastEditedAt\n#             }\n#             ... on IssueComment {\n#               id\n#               bodyText\n#               createdAt\n#               updatedAt\n#               lastEditedAt\n#             }\n#             ... on ClosedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#             ... on ReopenedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#             ... on Subscribable {\n#               id\n#               __typename\n#               viewerCanSubscribe\n#               viewerSubscription\n#             }\n#             ... on UnsubscribedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#             ... on MergedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               url\n#               resourcePath\n#               createdAt\n#             }\n#             ... on ReferencedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               commit {\n#                 url\n#                 commitUrl\n#               }\n#               isCrossRepository\n#               isDirectReference\n#               createdAt\n#             }\n#             ... on CrossReferencedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               isCrossRepository\n#               url\n#               resourcePath\n#               willCloseTarget\n#               referencedAt\n#               createdAt\n#             }\n#             ... on AssignedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               user {\n#                 name\n#                 login\n#               }\n#               createdAt\n#             }\n#             ... on UnassignedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               user {\n#                 name\n#                 login\n#               }\n#               createdAt\n#             }\n#             ... on LabeledEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               label {\n#                 name\n#                 description\n#                 color\n#               }\n#               createdAt\n#             }\n#             ... on UnlabeledEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               label {\n#                 name\n#                 description\n#                 color\n#               }\n#               createdAt\n#             }\n#             ... on MilestonedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               milestoneTitle\n#               createdAt\n#             }\n#             ... on DemilestonedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               milestoneTitle\n#               createdAt\n#             }\n#             ... on RenamedTitleEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               currentTitle\n#               previousTitle\n#               createdAt\n#             }\n#             ... on LockedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               lockReason\n#               createdAt\n#             }\n#             ... on UnlockedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#             ... on DeployedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#             }\n#             ... on DeploymentEnvironmentChangedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#             ... on HeadRefDeletedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               headRef {\n#                 id\n#                 name\n#                 prefix\n#               }\n#               headRefName\n#               createdAt\n#             }\n#             ... on HeadRefRestoredEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#             ... on HeadRefForcePushedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               afterCommit {\n#                 id\n#                 oid\n#               }\n#               beforeCommit {\n#                 id\n#                 oid\n#               }\n#               ref {\n#                 id\n#                 name\n#                 prefix\n#               }\n#               createdAt\n#             }\n#             ... on BaseRefForcePushedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               afterCommit {\n#                 id\n#                 oid\n#               }\n#               beforeCommit {\n#                 id\n#                 oid\n#               }\n#               ref {\n#                 id\n#                 name\n#                 prefix\n#               }\n#               createdAt\n#             }\n#             ... on ReviewRequestedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               requestedReviewer {\n#                 ...on User {\n#                   id\n#                   userName: name\n#                   userLogin: login\n#                 }\n#                 ...on Team {\n#                   id\n#                   teamName: name\n#                 }\n#               }\n#               createdAt\n#             }\n#             ... on ReviewRequestRemovedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               requestedReviewer {\n#                 ...on User {\n#                   id\n#                   userName: name\n#                   userLogin: login\n#                 }\n#                 ...on Team {\n#                   id\n#                   teamName: name\n#                 }\n#               }\n#               createdAt\n#             }\n#             ... on ReviewDismissedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               message\n#               resourcePath\n#               url\n#               previousReviewState\n#               createdAt\n#             }\n#           }\n#         }\n#         pageInfo {\n#           hasNextPage\n#           endCursor\n#         }\n#       }\n#     }\n#   }\n#   ...RateLimitInfo\n# }\n\n# # FullAttributes\n# query IssueTimelineItems($id: ID!, $first: Int = 100) {\n#   node(id: $id) {\n#     __typename\n#     ... on Issue {\n#       id\n#       author {\n#         __typename\n#         ...custom_actor\n#       }\n#       editor {\n#         __typename\n#         ...custom_actor\n#       }\n#       title\n#       body\n#       bodyText\n#       timeline(first: $first) {\n#         edges {\n#           node {\n#             __typename\n#             ... on Commit {\n#               ...custom_commit\n#             }\n#             ... on IssueComment {\n#               id\n#               author {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               bodyText\n#               createdAt\n#               lastEditedAt\n#               publishedAt\n#             }\n#             ... on ClosedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               url\n#               resourcePath\n#               createdAt\n#             }\n#             ... on ReopenedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#             ... on Subscribable {\n#               id\n#               __typename\n#               viewerCanSubscribe\n#               viewerSubscription\n#             }\n#             ... on UnsubscribedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#             ... on ReferencedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               commit {\n#                 url\n#                 commitUrl\n#               }\n#               isCrossRepository\n#               isDirectReference\n#               createdAt\n#             }\n#             ... on CrossReferencedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               isCrossRepository\n#               url\n#               resourcePath\n#               willCloseTarget\n#               referencedAt\n#               createdAt\n#             }\n#             ... on AssignedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               user {\n#                 name\n#                 login\n#               }\n#               createdAt\n#             }\n#             ... on UnassignedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               user {\n#                 name\n#                 login\n#               }\n#               createdAt\n#             }\n#             ... on LabeledEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               label {\n#                 name\n#                 description\n#                 color\n#               }\n#               createdAt\n#             }\n#             ... on UnlabeledEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               label {\n#                 name\n#                 description\n#                 color\n#               }\n#               createdAt\n#             }\n#             ... on MilestonedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               milestoneTitle\n#               createdAt\n#             }\n#             ... on DemilestonedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               milestoneTitle\n#               createdAt\n#             }\n#             ... on RenamedTitleEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               currentTitle\n#               previousTitle\n#               createdAt\n#             }\n#             ... on LockedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               lockReason\n#               createdAt\n#             }\n#             ... on UnlockedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#           }\n#         }\n#         pageInfo {\n#           hasNextPage\n#           endCursor\n#         }\n#       }\n#     }\n#   }\n#   ...RateLimitInfo\n# }\n" ;
pub const OPERATION_NAME: &'static str = "Issues";
#[allow(dead_code)]
type Boolean = bool;
#[allow(dead_code)]
type Float = f64;
#[allow(dead_code)]
type Int = i64;
#[allow(dead_code)]
type ID = String;
#[derive(Debug)]
pub enum IssueState {
    CLOSED,
    OPEN,
    Other(String),
}
impl ::serde::Serialize for IssueState {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            IssueState::CLOSED => "CLOSED",
            IssueState::OPEN => "OPEN",
            IssueState::Other(ref s) => &s,
        })
    }
}
impl<'de> ::serde::Deserialize<'de> for IssueState {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <String>::deserialize(deserializer)?;
        match s.as_str() {
            "CLOSED" => Ok(IssueState::CLOSED),
            "OPEN" => Ok(IssueState::OPEN),
            _ => Ok(IssueState::Other(s)),
        }
    }
}
#[derive(Deserialize, Debug)]
pub struct RateLimitInfo {
    #[serde(rename = "rateLimit")]
    pub rate_limit: Option<RateLimitInfoRateLimit>,
}
#[derive(Deserialize, Debug)]
pub struct RateLimitInfoRateLimit {
    pub cost: Int,
    pub limit: Int,
    pub remaining: Int,
    #[serde(rename = "resetAt")]
    pub reset_at: DateTime,
}
#[derive(Deserialize, Debug)]
pub struct RustIssuesRepositoryIssuesEdgesNodeRepository {
    pub id: ID,
}
#[derive(Deserialize, Debug)]
pub struct RustIssuesRepositoryIssuesEdgesNode {
    pub id: ID,
    pub number: Int,
    pub state: IssueState,
    pub title: String,
    #[serde(rename = "bodyText")]
    pub body_text: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    #[serde(rename = "lastEditedAt")]
    pub last_edited_at: Option<DateTime>,
    closed: Boolean,
    #[serde(rename = "closedAt")]
    pub closed_at: Option<DateTime>,
    pub repository: RustIssuesRepositoryIssuesEdgesNodeRepository,
}

impl RustIssuesRepositoryIssuesEdgesNode {
    pub fn closed(&self) -> i32 {
        match self.closed {
            true => 1,
            false => 0,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct RustIssuesRepositoryIssuesEdges {
    pub node: Option<RustIssuesRepositoryIssuesEdgesNode>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RustIssuesRepositoryIssuesPageInfo {
    #[serde(rename = "startCursor")]
    pub start_cursor: Option<String>,
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: Boolean,
    #[serde(rename = "hasPreviousPage")]
    pub has_previous_page: Boolean,
}
#[derive(Deserialize, Debug)]
pub struct RustIssuesRepositoryIssues {
    pub edges: Option<Vec<Option<RustIssuesRepositoryIssuesEdges>>>,
    #[serde(rename = "pageInfo")]
    pub page_info: RustIssuesRepositoryIssuesPageInfo,
}
#[derive(Deserialize, Debug)]
pub struct RustIssuesRepository {
    pub id: ID,
    pub url: URI,
    pub issues: RustIssuesRepositoryIssues,
}
#[derive(Serialize, Debug)]
pub struct Variables {
    pub owner: String,
    pub name: String,
    pub first: Option<Int>,
    pub states: Option<Vec<IssueState>>,
}
impl Variables {
    pub fn default_first() -> Option<Int> {
        Some(100i64)
    }
}
#[derive(Deserialize, Debug)]
pub struct ResponseData {
    pub repository: Option<RustIssuesRepository>,
    #[serde(flatten)]
    pub rate_limit_info: RateLimitInfo,
}

pub struct Issues;

impl<'de> graphql_client::GraphQLQuery<'de> for Issues {
    type Variables = Variables;
    type ResponseData = ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: QUERY,
            operation_name: OPERATION_NAME,
        }
    }
}
